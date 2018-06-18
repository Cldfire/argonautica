extern crate cbindgen;

use std::env;
// use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("target/argonautica.h");

    // let python_dir = Path::new("../argonautica-py/argonautica/");
    // if python_dir.exists() {
    //     let path = python_dir.join("argonautica.h");
    //     bindings.write_to_file(&path);
    // }
}
