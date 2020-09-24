use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        documentation_style: cbindgen::DocumentationStyle::C99,
        line_length: 100,
        style: cbindgen::Style::Type,
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Some(vec![String::from("owlchat-keystore")]),
            ..Default::default()
        },
        ..Default::default()
    };
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("binding.h");
}
