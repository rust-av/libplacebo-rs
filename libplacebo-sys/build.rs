extern crate bindgen;
extern crate system_deps;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder) -> String {
    builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*")
}

fn main() {
    let libs = system_deps::Config::new().probe().unwrap();
    let headers = libs.get("libplacebo").unwrap().include_paths.clone();

    let mut builder = bindgen::builder().header("data/placebo.h");

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    builder = builder.default_enum_style(bindgen::EnumVariation::Rust {
        non_exhaustive: false,
    });

    // Manually fix the comment so rustdoc won't try to pick them
    let s = format_write(builder);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut file = File::create(out_path.join("placebo.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
