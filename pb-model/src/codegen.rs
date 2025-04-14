use std::collections::HashMap;

use anyhow::{Context, Result, bail, ensure};
use quote::quote;

use crate::onnx;

#[derive(Default)]
pub struct Generator {
    vars: HashMap<String, Var>,
    stmts: Vec<syn::Stmt>,
}

#[derive(Debug, PartialEq, Eq)]
struct Var {
    ident: syn::Ident,
    elem_ty: Type,
    shape: Vec<usize>,
    ty: syn::Type,
    kind: VarKind,
}

#[derive(Debug, PartialEq, Eq)]
enum VarKind {
    Input,
    Constant,
    Output,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Type {
    F32,
    I64,
}

#[derive(Debug)]
enum Op {
    Gemm {
        alpha: f32,
        beta: f32,
        trans_a: bool,
        trans_b: bool,
    },
    Tanh,
    Shape {
        start: i64,
        end: Option<i64>,
    },
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(mut self, model: &onnx::ModelProto) -> Result<syn::File> {
        let graph = model.graph.as_ref().context("expected graph")?;

        let ident = to_ident(&graph.name);

        let mut inputs = Vec::new();
        for value_info in &graph.input {
            let arg = Var::from_value_info(value_info)?;
            inputs.push(arg.pat_type());
            self.add_var(&value_info.name, arg)?;
        }

        for tensor in &graph.initializer {
            self.generate_constant(tensor)?;
        }

        for node in &graph.node {
            let fn_item = match self.generate_node(node) {
                Ok(i) => i,
                _ => break,
            };
            self.stmts.push(syn::Stmt::Item(fn_item.into()));
        }

        let stmts = self.stmts;
        syn::parse2(quote!(fn #ident(#(#inputs),*) {
            #(#stmts)*
        }))
        .context("invalid fn")
    }

    fn generate_node(&mut self, node: &onnx::NodeProto) -> Result<syn::ItemFn> {
        let name = &node.name;

        let mut args = Vec::new();
        for name in &node.input {
            let var = self
                .vars
                .get(name)
                .with_context(|| format!("input '{name}' not found"))?;
            if var.kind == VarKind::Constant {
                continue;
            }

            args.push(var.pat_type());
        }

        let op = Op::from_node(&node)?;

        let mut rets = Vec::new();
        let output_types = op.output_types(
            node.input
                .iter()
                .map(|i| (self.vars[i].elem_ty, self.vars[i].shape.clone())),
        )?;
        ensure!(
            output_types.len() == node.output.len(),
            "output length mismatch for {name}"
        );
        for (name, (elem_ty, shape)) in node.output.iter().zip(output_types) {
            let ident = to_ident(name);

            let ty = elem_ty.array(&shape);
            self.vars.insert(
                name.clone(),
                Var {
                    ident,
                    ty: ty.clone(),
                    shape,
                    elem_ty,
                    kind: VarKind::Output,
                },
            );

            rets.push(ty);
        }

        let ident = to_ident(name);
        syn::parse2(quote!(fn #ident(#(#args),*) -> (#(#rets),*) {})).context("invalid fn")
    }

    fn generate_constant(&mut self, tensor: &onnx::TensorProto) -> Result<()> {
        ensure!(!tensor.raw_data.is_empty(), "only raw data is supported");
        ensure!(tensor.segment.is_none(), "segmented format not supported");
        ensure!(
            tensor.external_data.is_empty(),
            "external data not supported"
        );

        let ident = to_ident(&tensor.name);
        let elem_ty = Type::from_i32(tensor.data_type)?;
        let dims = tensor
            .dims
            .iter()
            .map(|&dim| Ok(usize::try_from(dim)?))
            .collect::<Result<Vec<_>>>()?;
        let ty = elem_ty.array(&dims);
        let expr = elem_ty.parse_data(&dims, &tensor.raw_data)?;

        let const_item: syn::ItemConst = syn::parse2(quote!(const #ident: #ty = #expr;))?;
        self.stmts.push(syn::Stmt::Item(const_item.into()));

        self.add_var(
            &tensor.name,
            Var {
                ident,
                elem_ty,
                shape: dims,
                ty,
                kind: VarKind::Constant,
            },
        )?;
        Ok(())
    }

    fn add_var(&mut self, name: &str, input: Var) -> Result<()> {
        ensure!(!name.is_empty(), "name is empty");

        if self.vars.insert(name.to_owned(), input).is_some() {
            bail!("duplicate var name '{name}'")
        }

        Ok(())
    }
}

impl Var {
    pub fn from_value_info(value_info: &onnx::ValueInfoProto) -> Result<Self> {
        let ty = value_info
            .r#type
            .as_ref()
            .context("expected type in ValueInfo")?;
        match &ty.value {
            Some(onnx::type_proto::Value::TensorType(tensor_ty)) => {
                let elem_ty = Type::from_i32(tensor_ty.elem_type)?;

                let shape = tensor_ty
                    .shape
                    .as_ref()
                    .context("expected shape in TypeProto.Tensor")?;

                let shape = shape
                    .dim
                    .iter()
                    .map(|dim| match &dim.value {
                        &Some(onnx::tensor_shape_proto::dimension::Value::DimValue(value)) => {
                            Ok(usize::try_from(value)?)
                        }
                        Some(onnx::tensor_shape_proto::dimension::Value::DimParam(param)) => {
                            bail!("dimension param '{param}' not supported")
                        }
                        None => bail!("expected dimension"),
                    })
                    .collect::<Result<Vec<_>>>()
                    .context("invalid dimension in TypeProto.Tensor")?;

                let ty = elem_ty.array(&shape);

                Ok(Var {
                    elem_ty,
                    shape,
                    ty,
                    ident: to_ident(&value_info.name),
                    kind: VarKind::Input,
                })
            }
            Some(value) => bail!("unsupported value in ValueInfo: {:?}", value),
            None => bail!("expected type value in ValueInfo"),
        }
    }

    pub fn pat_type(&self) -> syn::PatType {
        let ident = &self.ident;
        let ty = &self.ty;
        syn::parse2(quote!(#ident: #ty)).unwrap()
    }
}

impl Type {
    fn from_i32(data_type: i32) -> Result<Self> {
        match onnx::tensor_proto::DataType::try_from(data_type)? {
            onnx::tensor_proto::DataType::Float => Ok(Type::F32),
            ty => bail!("unsupported data type: {:?}", ty),
        }
    }

    fn array(&self, dims: &[usize]) -> syn::Type {
        let elem_ty: syn::Type = match self {
            Type::F32 => syn::parse2(quote!(f32)).unwrap(),
            Type::I64 => syn::parse2(quote!(i64)).unwrap(),
        };

        let mut ty = elem_ty;
        for dim in dims.iter().rev() {
            ty = syn::parse2(quote!([#ty; #dim])).unwrap();
        }

        ty
    }

    fn parse_data(&self, dims: &[usize], data: &[u8]) -> Result<syn::Expr> {
        ensure!(
            data.len() % self.size() == 0,
            "invalid data len {} for type {:?}",
            data.len(),
            self
        );

        let mut values: Vec<syn::Expr> = data
            .chunks(self.size())
            .map(|chunk| match self {
                Type::F32 => {
                    let float = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    syn::parse2(quote!(#float)).unwrap()
                }
                Type::I64 => {
                    let int = i64::from_le_bytes([
                        chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
                        chunk[7],
                    ]);
                    syn::parse2(quote!(#int)).unwrap()
                }
            })
            .collect();

        for &dim in dims.iter().rev() {
            ensure!(
                values.len() % dim == 0,
                "invalid data len {} for type {:?} and dims {:?}",
                data.len(),
                self,
                dims
            );

            values = values
                .chunks(dim)
                .map(|chunk| syn::parse2(quote!([#(#chunk),*])).unwrap())
                .collect();
        }

        ensure!(
            values.len() == 1,
            "invalid data len {} for type {:?} and dims {:?}",
            data.len(),
            self,
            dims
        );
        Ok(values.into_iter().next().unwrap())
    }

    fn size(&self) -> usize {
        match self {
            Type::F32 => 4,
            Type::I64 => 8,
        }
    }
}

impl Op {
    fn from_node(node: &onnx::NodeProto) -> Result<Self> {
        match node.op_type.as_str() {
            "Gemm" => Ok(Op::Gemm {
                alpha: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "alpha")
                    .map(|a| a.f)
                    .unwrap_or(1.0),
                beta: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "beta")
                    .map(|a| a.f)
                    .unwrap_or(1.0),
                trans_a: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "transA")
                    .map(|a| a.i != 0)
                    .unwrap_or(false),
                trans_b: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "transB")
                    .map(|a| a.i != 0)
                    .unwrap_or(false),
            }),
            "Tanh" => Ok(Op::Tanh),
            "Shape" => Ok(Op::Shape {
                start: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "transB")
                    .map(|a| a.i)
                    .unwrap_or(0),
                end: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "transB")
                    .map(|a| a.i),
            }),
            op => bail!("unsupported operation '{op}'"),
        }
    }

    fn output_types(
        &self,
        inputs: impl Iterator<Item = (Type, Vec<usize>)>,
    ) -> Result<Vec<(Type, Vec<usize>)>> {
        let inputs: Vec<_> = inputs.collect();

        match *self {
            Op::Gemm {
                trans_a, trans_b, ..
            } => {
                ensure!(
                    2 <= inputs.len() && inputs.len() <= 3,
                    "invalid inputs len {} for Gemm operation",
                    inputs.len()
                );
                ensure!(
                    inputs[0].1.len() == 2,
                    "invalid len {} for input tensor A of Gemm operation",
                    inputs[0].1.len()
                );
                ensure!(
                    inputs[1].1.len() == 2,
                    "invalid len {} for input tensor B of Gemm operation",
                    inputs[1].1.len()
                );
                ensure!(
                    inputs[0].0 == inputs[1].0,
                    "type mismatch for Gemm operation"
                );

                let m = if trans_a {
                    inputs[0].1[1]
                } else {
                    inputs[0].1[0]
                };
                let n = if trans_b {
                    inputs[1].1[0]
                } else {
                    inputs[1].1[1]
                };

                Ok(vec![(inputs[0].0, vec![m, n])])
            }
            Op::Tanh => {
                ensure!(
                    inputs.len() == 1,
                    "invalid inputs len {} for Tanh operation",
                    inputs.len()
                );

                Ok(inputs)
            }
            Op::Shape { .. } => Ok(vec![(Type::I64, vec![1])]),
        }
    }
}

fn to_ident(s: &str) -> syn::Ident {
    let s: String = s
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect();
    syn::parse_str(&s).unwrap()
}
