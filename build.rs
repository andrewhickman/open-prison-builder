mod atlas;

extern crate embed_resource;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("cargo::rerun-if-changed=build/windows/icon.rc");
        println!("cargo::rerun-if-changed=build/windows/icon_1024x1024.png");
        embed_resource::compile("build/windows/icon.rc");
    }

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    atlas::write_atlas(manifest.join("assets/textures/atlas.png"));
}
