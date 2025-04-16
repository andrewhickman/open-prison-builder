use std::{
    collections::{HashMap, HashSet},
    mem::replace,
};

use anyhow::{Context, Result, bail, ensure};
use quote::quote;

use crate::{
    onnx,
    op::{Operation, Var},
    tensor::{ElementType, Tensor, TensorType, wrap_index},
};

#[derive(Default)]
pub struct Generator {
    vars: HashMap<String, Var>,
    operations: HashMap<String, Operation>,
    outputs: HashSet<String>,
    stmts: Vec<syn::Stmt>,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(mut self, model: &onnx::ModelProto) -> Result<syn::File> {
        let graph = model.graph.as_ref().context("expected graph")?;

        for value_info in &graph.input {
            self.add_input(value_info)
                .with_context(|| format!("failed to process input {}", value_info.name))?;
        }

        for tensor in &graph.initializer {
            self.add_constant(tensor)
                .with_context(|| format!("failed to process initializer {}", tensor.name))?;
        }

        for node in &graph.node {
            self.add_node(node)
                .with_context(|| format!("failed to process node {}", node.name))?;
        }

        for output in &graph.output {
            self.outputs.insert(output.name.clone());
        }

        for node in graph.node.iter().rev() {
            self.output_node(node);
        }

        for tensor in &graph.initializer {
            self.output_constant(tensor);
        }

        for input in &graph.input {
            self.outputs.remove(&input.name);
        }
        ensure!(
            self.outputs.is_empty(),
            "unfulfilled outputs: {:?}",
            self.outputs
        );

        let ident = self.output_ident(&graph.name);
        let inputs: Vec<syn::PatType> = graph
            .input
            .iter()
            .map(|i| {
                let ident = self.output_ident(&i.name);
                let ty = self.output_tensor_type(self.vars[&i.name].ty());
                syn::parse2(quote!(#ident: #ty)).unwrap()
            })
            .collect();
        let output_idents: Vec<syn::Ident> = graph
            .output
            .iter()
            .map(|i| self.output_ident(&i.name))
            .collect();
        let output_tys: Vec<syn::Type> = graph
            .output
            .iter()
            .map(|i| self.output_tensor_type(self.vars[&i.name].ty()))
            .collect();

        self.stmts.reverse();
        let stmts = self.stmts;

        Ok(syn::parse2(quote!(
            #[allow(
                unused_parens,
                non_snake_case,
                non_upper_case_globals,
                clippy::let_and_return,
                clippy::just_underscores_and_digits
            )]
            pub fn #ident(#(#inputs),*) -> #(#output_tys),* {
                #(#stmts)*

                #(#output_idents),*
            }
        ))
        .unwrap())
    }

    fn add_input(&mut self, value_info: &onnx::ValueInfoProto) -> Result<()> {
        let ty = TensorType::from_value_info_proto(value_info)?;
        self.add_var(&value_info.name, Var::Input(ty))
    }

    fn add_constant(&mut self, tensor: &onnx::TensorProto) -> Result<()> {
        let name = &tensor.name;
        let tensor = Tensor::from_proto(tensor)?;

        self.add_var(name, Var::Const(tensor))
    }

    fn add_node(&mut self, node: &onnx::NodeProto) -> Result<()> {
        let inputs = node
            .input
            .iter()
            .map(|input| {
                self.vars
                    .get(input)
                    .cloned()
                    .with_context(|| format!("input '{input}' not found"))
            })
            .collect::<Result<Vec<Var>>>()?;

        let op = Operation::from_proto(node)?;

        let outputs = op.apply(&inputs)?;
        ensure!(
            outputs.len() == node.output.len(),
            "output length mismatch for {}",
            node.name
        );
        for (name, var) in node.output.iter().zip(outputs) {
            self.add_var(name, var)?;
        }

        if self.operations.insert(node.name.clone(), op).is_some() {
            bail!("duplicate node '{}'", node.name);
        }

        Ok(())
    }

    fn add_var(&mut self, name: &str, input: Var) -> Result<()> {
        ensure!(!name.is_empty(), "name is empty");

        if self.vars.insert(name.to_owned(), input).is_some() {
            bail!("duplicate var name '{name}'")
        }

        Ok(())
    }

    fn output_node(&mut self, node: &onnx::NodeProto) {
        if node
            .output
            .iter()
            .all(|output| !self.outputs.contains(output))
        {
            return;
        }

        for output in &node.output {
            if let Var::Const(tensor) = &self.vars[output] {
                if self.outputs.remove(output) {
                    let item = self.output_constant_value(output, tensor);
                    self.stmts.push(syn::Stmt::Item(item));
                }
            }
        }

        if node
            .output
            .iter()
            .all(|output| !self.outputs.contains(output))
        {
            return;
        }

        let mut output_bindings = Vec::new();
        for output in &node.output {
            if self.outputs.remove(output) {
                let ident = self.output_ident(output);
                output_bindings.push(quote!(#ident));
            } else {
                output_bindings.push(quote!(_));
            };
        }

        let op_expr =
            self.output_operation(&self.operations[&node.name], &node.input, &node.output);

        let local: syn::Stmt =
            syn::parse2(quote!(let (#(#output_bindings),*) = #op_expr;)).unwrap();
        self.stmts.push(local);

        for input in &node.input {
            self.outputs.insert(input.clone());
        }
    }

    fn output_constant(&mut self, tensor: &onnx::TensorProto) {
        let name = &tensor.name;
        if self.outputs.remove(name) {
            let Var::Const(tensor) = &self.vars[name] else {
                panic!("expected const");
            };

            let item = self.output_constant_value(name, tensor);
            self.stmts.push(syn::Stmt::Item(item));
        }
    }

    fn output_index_expr(&self, input: &str, indices: &[usize]) -> syn::Expr {
        let ty = self.vars[input].ty();
        let ident = self.output_ident(input);
        let mut expr: syn::Expr = syn::parse2(quote!(#ident)).unwrap();
        for (axis, &index) in indices.iter().enumerate() {
            let broadcast_index = match ty.dim(axis as i64 - indices.len() as i64) {
                None => continue,
                Some(1) => 0,
                Some(_) => index,
            };

            expr = syn::parse2(quote!(#expr[#broadcast_index])).unwrap();
        }
        expr
    }

    fn output_operation(&self, op: &Operation, inputs: &[String], outputs: &[String]) -> syn::Expr {
        match *op {
            // TODO: consider using SIMD instructions here
            Operation::Gemm {
                alpha,
                beta,
                trans_a,
                trans_b,
            } => {
                let k = if trans_a {
                    self.vars[&inputs[0]].shape()[0]
                } else {
                    self.vars[&inputs[0]].shape()[1]
                };

                self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                    let (m, n) = (indices[0], indices[1]);

                    let terms: Vec<syn::Expr> = (0..k)
                        .map(|k| {
                            let a_term = if trans_a {
                                self.output_index_expr(&inputs[0], &[k, m])
                            } else {
                                self.output_index_expr(&inputs[0], &[m, k])
                            };
                            let b_term = if trans_b {
                                self.output_index_expr(&inputs[1], &[n, k])
                            } else {
                                self.output_index_expr(&inputs[1], &[k, n])
                            };

                            syn::parse2(quote!(#a_term * #b_term)).unwrap()
                        })
                        .collect();

                    let c_term = self.output_index_expr(&inputs[2], indices);

                    syn::parse2(quote!(#alpha * (#(#terms)+*) + #beta * #c_term)).unwrap()
                })
            }
            Operation::Tanh => self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                let input = self.output_index_expr(&inputs[0], indices);
                syn::parse2(quote!(#input.tanh())).unwrap()
            }),
            Operation::Shape { .. } => unreachable!(),
            Operation::Constant { .. } => unreachable!(),
            Operation::Gather { .. } => unimplemented!(),
            Operation::Add => unimplemented!(),
            Operation::Div => unimplemented!(),
            Operation::Mul => unimplemented!(),
            Operation::Slice => {
                let data = &self.vars[&inputs[0]];
                let starts = self.vars[&inputs[1]].unwrap_const();
                let axes = self.vars[&inputs[3]].unwrap_const();

                let mut start_indices = vec![0; data.rank()];

                for i in 0..axes.dim(0).unwrap() {
                    let start = starts.index_i64(&[i]);
                    let axis = axes.index_i64(&[i]);

                    let axis_index = wrap_index(axis, data.rank()).unwrap();
                    let start = wrap_index(start, data.shape()[axis_index]).unwrap();

                    start_indices[axis_index] = start;
                }

                self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                    let data_indices: Vec<_> = start_indices
                        .iter()
                        .zip(indices)
                        .map(|(&start, &index)| start + index)
                        .collect();
                    self.output_index_expr(&inputs[0], &data_indices)
                })
            }
            Operation::Max => self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                inputs
                    .iter()
                    .map(|input| self.output_index_expr(input, indices))
                    .reduce(|l, r| syn::parse2(quote!(#l.max(#r))).unwrap())
                    .unwrap()
            }),
            Operation::Min => self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                inputs
                    .iter()
                    .map(|input| self.output_index_expr(input, indices))
                    .reduce(|l, r| syn::parse2(quote!(#l.min(#r))).unwrap())
                    .unwrap()
            }),
            Operation::Concat { axis } => {
                let axis_index = wrap_index(axis, self.vars[&inputs[0]].rank()).unwrap();
                let axis_dims: Vec<usize> = inputs
                    .iter()
                    .map(|input| self.vars[input].ty().dim(axis).unwrap())
                    .scan(0, |sum, dim| Some(replace(sum, *sum + dim)))
                    .collect();

                self.output_tensor_from_fn(self.vars[&outputs[0]].ty(), |indices| {
                    let mut indices = indices.to_vec();
                    let (input, index) = match axis_dims.binary_search(&indices[axis_index]) {
                        Ok(i) => (&inputs[i], 0),
                        Err(i) => (&inputs[i - 1], indices[axis_index] - axis_dims[i - 1]),
                    };
                    indices[axis_index] = index;

                    self.output_index_expr(input, &indices)
                })
            }
        }
    }

    fn output_constant_value(&self, name: &str, tensor: &Tensor) -> syn::Item {
        let ident = self.output_ident(name);
        let ty = self.output_tensor_type(tensor.ty());
        let expr = self.output_tensor(tensor);
        syn::parse2(quote!(const #ident: #ty = #expr;)).unwrap()
    }

    fn output_tensor_from_fn(
        &self,
        ty: &TensorType,
        f: impl Fn(&[usize]) -> syn::Expr,
    ) -> syn::Expr {
        let exprs = ty.indices().map(|indices| f(&indices)).collect();
        self.output_tensor_items(exprs, ty.shape())
    }

    fn output_tensor_items(&self, mut exprs: Vec<syn::Expr>, shape: &[usize]) -> syn::Expr {
        for &dim in shape.iter().rev() {
            exprs = exprs
                .chunks(dim)
                .map(|chunk| syn::parse2(quote!([#(#chunk),*])).unwrap())
                .collect();
        }

        assert_eq!(exprs.len(), 1);
        exprs.into_iter().next().unwrap()
    }

    fn output_tensor(&self, tensor: &Tensor) -> syn::Expr {
        let values: Vec<syn::Expr> = match tensor.elem_ty() {
            ElementType::F32 => tensor
                .iter_f32()
                .map(|f| syn::parse2(quote!(#f)).unwrap())
                .collect(),
            ElementType::I64 => tensor
                .iter_i64()
                .map(|i| syn::parse2(quote!(#i)).unwrap())
                .collect(),
        };

        self.output_tensor_items(values, tensor.shape())
    }

    fn output_tensor_type(&self, tensor_ty: &TensorType) -> syn::Type {
        let mut ty = self.output_element_type(tensor_ty.elem_ty());
        for &dim in tensor_ty.shape().iter().rev() {
            ty = syn::parse2(quote!([#ty; #dim])).unwrap();
        }
        ty
    }

    fn output_element_type(&self, elem_ty: ElementType) -> syn::Type {
        match elem_ty {
            ElementType::F32 => syn::parse2(quote!(f32)).unwrap(),
            ElementType::I64 => syn::parse2(quote!(i64)).unwrap(),
        }
    }

    fn output_ident(&self, s: &str) -> syn::Ident {
        let mut s: String = s
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
            .collect();
        if s.chars().next().unwrap().is_ascii_digit() {
            s.insert(0, '_');
        }
        syn::parse_str(&s).unwrap()
    }
}
