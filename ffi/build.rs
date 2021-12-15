use std::collections::HashMap;
use std::{env, fs};

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=def.proto");
    println!("cargo:rerun-if-changed=src/lib.rs");

    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["def.proto"], &["."])
        .and(fs::remove_file("src/pb/google.protobuf.rs").or(Ok(())))
        .expect("Failed to compile protobuf");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config {
        language: cbindgen::Language::Cxx,
        documentation_style: cbindgen::DocumentationStyle::Cxx,
        line_length: 150,
        style: cbindgen::Style::Both,
        no_includes: true,
        sys_includes: vec![String::from("stdint.h")],
        defines: HashMap::from([(
            String::from("feature = dart-ffi"),
            String::from("DEFINE_DART"),
        )]),
        ..Default::default()
    };
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("binding.h");
    // run cargo fmt to format the generated protobuf
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("fmt");
    cmd.status().expect("Failed to run cargo fmt");
}
