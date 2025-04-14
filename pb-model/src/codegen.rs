use std::collections::HashMap;

use anyhow::{Context, Result, bail, ensure};
use quote::quote;

use crate::{
    onnx,
    op::{Operation, Var},
    tensor::{Tensor, TensorType},
};

#[derive(Default)]
pub struct Generator {
    vars: HashMap<String, Var>,
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

        // syn::parse2(quote!(fn #ident(#(#inputs),*) {
        //     #(#stmts)*
        // }))
        // .context("invalid fn")

        todo!()
    }

    fn add_input(&mut self, value_info: &onnx::ValueInfoProto) -> Result<()> {
        let ty = TensorType::from_value_info_proto(value_info)?;
        self.add_var(&value_info.name, Var::Input(ty))
    }

    fn add_constant(&mut self, tensor: &onnx::TensorProto) -> Result<()> {
        let name = &tensor.name;
        let tensor = Tensor::from_proto(tensor)?;

        // let const_item: syn::ItemConst = syn::parse2(quote!(const #ident: #ty = #expr;))?;
        // self.stmts.push(syn::Stmt::Item(const_item.into()));

        self.add_var(name, Var::Const(tensor))
    }

    fn add_node(&mut self, node: &onnx::NodeProto) -> Result<()> {
        let name = &node.name;

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
            "output length mismatch for {name}"
        );
        for (name, var) in node.output.iter().zip(outputs) {
            self.add_var(name, var);
        }

        // syn::parse2(quote!(fn #ident(#(#args),*) -> (#(#rets),*) {})).context("invalid fn")
        todo!()
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
    pub fn pat_type(&self) -> syn::PatType {
        // let ident = &self.ident;
        // let ty = &self.ty_syntax;
        // syn::parse2(quote!(#ident: #ty)).unwrap()
        todo!()
    }
}

// impl TensorType {
//     fn syntax(&self, dims: &[usize]) -> syn::Type {
//         let elem_ty: syn::Type = match self {
//             Type::F32 => syn::parse2(quote!(f32)).unwrap(),
//             Type::I64 => syn::parse2(quote!(i64)).unwrap(),
//         };

//         let mut ty = elem_ty;
//         for dim in dims.iter().rev() {
//             ty = syn::parse2(quote!([#ty; #dim])).unwrap();
//         }

//         ty
//     }

//     fn parse_data(&self, dims: &[usize], data: &[u8]) -> Result<syn::Expr> {
//         ensure!(
//             data.len() % self.size() == 0,
//             "invalid data len {} for type {:?}",
//             data.len(),
//             self
//         );

//         let mut values: Vec<syn::Expr> = data
//             .chunks(self.size())
//             .map(|chunk| match self {
//                 Type::F32 => {
//                     let float = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
//                     syn::parse2(quote!(#float)).unwrap()
//                 }
//                 Type::I64 => {
//                     let int = i64::from_le_bytes([
//                         chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
//                         chunk[7],
//                     ]);
//                     syn::parse2(quote!(#int)).unwrap()
//                 }
//             })
//             .collect();

//         for &dim in dims.iter().rev() {
//             ensure!(
//                 values.len() % dim == 0,
//                 "invalid data len {} for type {:?} and dims {:?}",
//                 data.len(),
//                 self,
//                 dims
//             );

//             values = values
//                 .chunks(dim)
//                 .map(|chunk| syn::parse2(quote!([#(#chunk),*])).unwrap())
//                 .collect();
//         }

//         ensure!(
//             values.len() == 1,
//             "invalid data len {} for type {:?} and dims {:?}",
//             data.len(),
//             self,
//             dims
//         );
//         Ok(values.into_iter().next().unwrap())
//     }

//     fn size(&self) -> usize {
//         match self {
//             Type::F32 => 4,
//             Type::I64 => 8,
//         }
//     }
// }

fn to_ident(s: &str) -> syn::Ident {
    let s: String = s
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect();
    syn::parse_str(&s).unwrap()
}

fn multidirectional_broadcast_output_dims(left: &[usize], right: &[usize]) -> Result<Vec<usize>> {
    (0..left.len().max(right.len()))
        .map(|i| {
            match (
                left.get(left.len() - 1 - i).copied().unwrap_or(1),
                right.get(right.len() - 1 - i).copied().unwrap_or(1),
            ) {
                (1, r) => Ok(r),
                (l, 1) => Ok(l),
                (r, l) if l == r => Ok(l),
                (_, _) => bail!(
                    "incompatible multidirectional broadcast dimensions: {left:?} and {right:?}"
                ),
            }
        })
        .rev()
        .collect()
}

fn eq_or_one(left: usize, right: usize) -> bool {
    left == right || left == 1 || right == 1
}
