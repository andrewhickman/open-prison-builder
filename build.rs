use std::env;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("cargo::rerun-if-changed=build/windows/icon.rc");
        println!("cargo::rerun-if-changed=build/icon.png");
        embed_resource::compile("build/windows/icon.rc");
    }
}
