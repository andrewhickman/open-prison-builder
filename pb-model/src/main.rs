use std::{fs, path::Path};

use prost_reflect::{DescriptorPool, DynamicMessage};

pub mod onnx {
    include!(concat!(env!("OUT_DIR"), "/", "onnx.rs"));
}

pub fn main() {
    let fd_set = DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/", "file_descriptor_set.pb")).as_slice(),
    )
    .unwrap();

    let model_bytes =
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../pb-learn/model.pb")).unwrap();

    let model_proto = fd_set.get_message_by_name("onnx.ModelProto").unwrap();

    let model_message = DynamicMessage::decode(model_proto, model_bytes.as_slice()).unwrap();

    let model_json = serde_json::to_string_pretty(&model_message).unwrap();

    fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../pb-learn/model.json"),
        model_json,
    )
    .unwrap();
}
