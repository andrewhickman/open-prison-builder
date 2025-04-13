use std::{env, fs, path::PathBuf};

use prost::Message;

fn main() {
    let descriptors = protox::compile(["onnx.proto"], ["onnx/onnx"]).unwrap();

    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("file_descriptor_set.pb"),
        descriptors.encode_to_vec(),
    )
    .unwrap();

    prost_build::Config::new().compile_fds(descriptors).unwrap();
}
