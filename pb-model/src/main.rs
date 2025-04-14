mod codegen;

use std::path::Path;

use anyhow::{Context, Result};
use prost::Message;

use self::codegen::Generator;

pub mod onnx {
    include!(concat!(env!("OUT_DIR"), "/", "onnx.rs"));
}

pub fn main() -> Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let model_bytes = fs_err::read(manifest_dir.join("../pb-learn/model.pb"))?;
    let model =
        onnx::ModelProto::decode(model_bytes.as_slice()).context("failed to decode model proto")?;

    let file = Generator::new()
        .generate(&model)
        .context("failed to generate code")?;
    let file = prettyplease::unparse(&file);
    fs_err::write(manifest_dir.join("out.rs"), file)?;

    Ok(())
}
