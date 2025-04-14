use anyhow::{Context, Result, bail};

use crate::{
    onnx,
    tensor::{Tensor, TensorType},
};

#[derive(Debug, Clone)]
pub enum Var {
    Input(TensorType),
    Const(Tensor),
}

#[derive(Debug)]
pub enum Operation {
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
    Constant {
        value: Tensor,
    },
    Gather {
        axis: i64,
    },
    Add,
    Div,
    Mul,
    Slice,
}

impl Operation {
    pub fn from_proto(node: &onnx::NodeProto) -> Result<Self> {
        match node.op_type.as_str() {
            "Gemm" => Ok(Operation::Gemm {
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
            "Tanh" => Ok(Operation::Tanh),
            "Shape" => Ok(Operation::Shape {
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
            "Constant" => {
                let tensor = node
                    .attribute
                    .iter()
                    .find(|a| a.name == "value")
                    .and_then(|a| a.t.as_ref())
                    .context("unsupported constant type")?;
                Ok(Operation::Constant {
                    value: Tensor::from_proto(tensor)?,
                })
            }
            "Gather" => Ok(Operation::Gather {
                axis: node
                    .attribute
                    .iter()
                    .find(|a| a.name == "axis")
                    .map(|a| a.i)
                    .unwrap_or(0),
            }),
            "Add" => Ok(Operation::Add),
            "Div" => Ok(Operation::Div),
            "Mul" => Ok(Operation::Mul),
            "Slice" => Ok(Operation::Slice),
            op => bail!("unsupported operation '{op}'"),
        }
    }

    pub fn apply(&self, _inputs: &[Var]) -> Result<Vec<Var>> {
        // match *self {
        //     Op::Gemm {
        //         trans_a, trans_b, ..
        //     } => {
        //         ensure!(
        //             2 <= inputs.len() && inputs.len() <= 3,
        //             "invalid inputs len {} for Gemm operation",
        //             inputs.len()
        //         );
        //         ensure!(
        //             inputs[0].1.len() == 2,
        //             "invalid len {} for input tensor A of Gemm operation",
        //             inputs[0].1.len()
        //         );
        //         ensure!(
        //             inputs[1].1.len() == 2,
        //             "invalid len {} for input tensor B of Gemm operation",
        //             inputs[1].1.len()
        //         );
        //         ensure!(
        //             inputs[0].0 == inputs[1].0,
        //             "type mismatch for Gemm operation"
        //         );

        //         let m = if trans_a {
        //             inputs[0].1[1]
        //         } else {
        //             inputs[0].1[0]
        //         };
        //         let n = if trans_b {
        //             inputs[1].1[0]
        //         } else {
        //             inputs[1].1[1]
        //         };

        //         Ok(vec![(inputs[0].0, vec![m, n])])
        //     }
        //     Op::Tanh => {
        //         ensure!(
        //             inputs.len() == 1,
        //             "invalid inputs len {} for Tanh operation",
        //             inputs.len()
        //         );

        //         Ok(inputs)
        //     }
        //     Op::Shape { .. } => Ok(vec![(Type::I64, vec![1])]),
        //     Op::Constant { value, .. } => {
        //         ensure!(
        //             inputs.is_empty(),
        //             "invalid inputs len {} for Constant operation",
        //             inputs.len()
        //         );

        //         Ok(vec![(value.ty(), shape.clone())])
        //     }
        //     Op::Gather { axis } => {
        //         ensure!(
        //             inputs.len() == 2,
        //             "invalid inputs len {} for Gather operation",
        //             inputs.len()
        //         );
        //         ensure!(
        //             -(inputs.len() as i64) <= axis && axis < inputs.len() as i64,
        //             "invalid axis {} for Gather operation on tensor of rank {}",
        //             axis,
        //             inputs[0].1.len()
        //         );

        //         let axis_index = if axis < 0 {
        //             inputs.len() as i64 - axis
        //         } else {
        //             axis
        //         } as usize;

        //         let mut output_dim = inputs[0].1.clone();
        //         output_dim
        //             .splice(axis_index..(axis_index + 1), inputs[1].1.iter().copied())
        //             .for_each(drop);

        //         Ok(vec![(inputs[0].0, output_dim)])
        //     }
        //     Op::Add | Op::Div | Op::Mul => {
        //         ensure!(
        //             inputs.len() <= 2,
        //             "invalid inputs len {} for binary operation",
        //             inputs.len()
        //         );
        //         ensure!(
        //             inputs[0].0 == inputs[1].0,
        //             "type mismatch for binary operation"
        //         );

        //         let output_dim =
        //             multidirectional_broadcast_output_dims(&inputs[0].1, &inputs[1].1)?;
        //         Ok(vec![(inputs[0].0, output_dim)])
        //     }
        //     Op::Slice => {
        //         ensure!(
        //             3 <= inputs.len() && inputs.len() <= 5,
        //             "invalid inputs len {} for Slice operation",
        //             inputs.len()
        //         );
        //         ensure!(
        //             inputs.len() == 4,
        //             "unsupported len {} for Slice operation",
        //             inputs.len()
        //         );
        //         ensure!(
        //             inputs[3].1.len() == 1,
        //             "axes must be a 1D tensor for Slice operation, got {:?}",
        //             inputs[3].1
        //         );
        //         ensure!(
        //             inputs[1].1 == inputs[3].1,
        //             "mismatch of starts and axes dimensions for Slice operation"
        //         );
        //         ensure!(
        //             inputs[2].1 == inputs[3].1,
        //             "mismatch of ends and axes dimensions for Slice operation"
        //         );

        //         // let output_dim = (0..inputs[3].1.len())
        //         //     .map(|i|)

        //         todo!()
        //     }
        // }

        todo!()
    }
}
