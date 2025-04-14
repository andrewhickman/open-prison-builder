use anyhow::{Context, Result, bail, ensure};
use prost::bytes::Bytes;

use crate::onnx::{self, tensor_proto};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ElementType {
    F32,
    I64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TensorType {
    pub elem_ty: ElementType,
    pub shape: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Tensor {
    ty: TensorType,
    data: Bytes,
}

impl ElementType {
    pub fn from_proto(ty: tensor_proto::DataType) -> Result<Self> {
        match ty {
            tensor_proto::DataType::Float => Ok(ElementType::F32),
            tensor_proto::DataType::Int64 => Ok(ElementType::I64),
            _ => bail!("unsupported tensor type: {ty:?}"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ElementType::F32 => 4,
            ElementType::I64 => 8,
        }
    }
}

impl TensorType {
    pub fn from_tensor_proto(tensor: &onnx::TensorProto) -> Result<Self> {
        let elem_ty = ElementType::from_proto(tensor.data_type.try_into()?)?;
        let shape = tensor
            .dims
            .iter()
            .map(|&dim| Ok(usize::try_from(dim)?))
            .collect::<Result<Vec<_>>>()?;

        Ok(TensorType { elem_ty, shape })
    }

    pub fn from_value_info_proto(value_info: &onnx::ValueInfoProto) -> Result<Self> {
        let ty = value_info
            .r#type
            .as_ref()
            .context("expected type in ValueInfo")?;
        match &ty.value {
            Some(onnx::type_proto::Value::TensorType(tensor_ty)) => {
                let elem_ty = ElementType::from_proto(tensor_ty.elem_type.try_into()?)?;

                let shape = tensor_ty
                    .shape
                    .as_ref()
                    .context("expected shape in TypeProto.Tensor")?
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

                Ok(TensorType { elem_ty, shape })
            }
            Some(value) => bail!("unsupported value in ValueInfo: {:?}", value),
            None => bail!("expected type value in ValueInfo"),
        }
    }

    pub fn len(&self) -> usize {
        self.elem_ty.len() * self.shape.iter().product::<usize>()
    }
}

impl Tensor {
    pub fn new(ty: TensorType, data: Bytes) -> Result<Self> {
        ensure!(
            data.len() == ty.len(),
            "invalid data len {} for type {:?}",
            data.len(),
            ty,
        );

        Ok(Tensor { ty, data })
    }

    pub fn from_proto(tensor: &onnx::TensorProto) -> Result<Self> {
        ensure!(!tensor.raw_data.is_empty(), "only raw data is supported");
        ensure!(tensor.segment.is_none(), "segmented format not supported");
        ensure!(
            tensor.external_data.is_empty(),
            "external data not supported"
        );

        let shape = TensorType::from_tensor_proto(tensor)?;
        let data = tensor.raw_data.clone();

        Tensor::new(shape, data)
    }

    pub fn elem_ty(&self) -> ElementType {
        self.ty.elem_ty
    }

    pub fn shape(&self) -> &[usize] {
        &self.ty.shape
    }

    pub fn ty(&self) -> &TensorType {
        &self.ty
    }
}
