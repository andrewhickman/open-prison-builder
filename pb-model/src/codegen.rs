use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail, ensure};
use quote::quote;

use crate::{
    onnx,
    op::{Operation, Var},
    tensor::{ElementType, Tensor, TensorType},
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

        let ident = output_ident(&graph.name);
        let inputs: Vec<syn::PatType> = graph
            .input
            .iter()
            .map(|i| {
                let ident = output_ident(&i.name);
                let ty = output_tensor_type(self.vars[&i.name].ty());
                syn::parse2(quote!(#ident: #ty)).unwrap()
            })
            .collect();
        let output_idents: Vec<syn::Ident> =
            graph.output.iter().map(|i| output_ident(&i.name)).collect();
        let output_tys: Vec<syn::Type> = graph
            .output
            .iter()
            .map(|i| output_tensor_type(self.vars[&i.name].ty()))
            .collect();

        self.stmts.reverse();
        let stmts = self.stmts;

        Ok(syn::parse2(quote!(

            fn #ident(#(#inputs),*) -> #(#output_tys),* {
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
                    let ident = output_ident(output);
                    let ty = output_tensor_type(tensor.ty());
                    let expr = output_tensor(tensor);
                    let const_expr: syn::Item =
                        syn::parse2(quote!(const #ident: #ty = #expr;)).unwrap();
                    self.stmts.push(syn::Stmt::Item(const_expr));

                    self.outputs.remove(output);
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
                let ident = output_ident(output);
                output_bindings.push(quote!(#ident));
            } else {
                output_bindings.push(quote!(_));
            };
        }

        let op_expr = output_operation(&self.operations[&node.name], &node.input);

        let local: syn::Stmt = syn::parse2(quote!(let #(#output_bindings),* = #op_expr;)).unwrap();
        self.stmts.push(local);

        for input in &node.input {
            self.outputs.insert(input.clone());
        }
    }
}

fn output_operation(_op: &Operation, _inputs: &[String]) -> syn::Expr {
    syn::parse2(quote!(unimplemented!())).unwrap()
}

fn output_tensor(tensor: &Tensor) -> syn::Expr {
    let mut values: Vec<syn::Expr> = match tensor.elem_ty() {
        ElementType::F32 => tensor
            .iter_f32()
            .map(|f| syn::parse2(quote!(#f)).unwrap())
            .collect(),
        ElementType::I64 => tensor
            .iter_i64()
            .map(|i| syn::parse2(quote!(#i)).unwrap())
            .collect(),
    };

    for &dim in tensor.shape().iter().rev() {
        values = values
            .chunks(dim)
            .map(|chunk| syn::parse2(quote!([#(#chunk),*])).unwrap())
            .collect();
    }

    assert_eq!(values.len(), 1);
    values.into_iter().next().unwrap()
}

fn output_tensor_type(tensor_ty: &TensorType) -> syn::Type {
    let mut ty = output_element_type(tensor_ty.elem_ty());
    for &dim in tensor_ty.shape().iter().rev() {
        ty = syn::parse2(quote!([#ty; #dim])).unwrap();
    }
    ty
}

fn output_element_type(elem_ty: ElementType) -> syn::Type {
    match elem_ty {
        ElementType::F32 => syn::parse2(quote!(f32)).unwrap(),
        ElementType::I64 => syn::parse2(quote!(f64)).unwrap(),
    }
}

fn output_ident(s: &str) -> syn::Ident {
    let mut s: String = s
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect();
    if s.chars().next().unwrap().is_ascii_digit() {
        s.insert(0, '_');
    }
    syn::parse_str(&s).unwrap()
}
