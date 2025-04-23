use std::{env, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let movement_model = manifest_dir.join("../pb-learn/train/models/movement.onnx");

    println!("cargo::rerun-if-changed={}", movement_model.display());
    pb_learn_model::generate_model(
        &movement_model,
        &manifest_dir.join("src/pawn/ai/path/model.rs"),
    )
}
