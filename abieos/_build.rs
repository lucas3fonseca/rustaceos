// extern crate bindgen;

// use std::path::PathBuf;

// fn main() {

//     println!("cargo:rerun-if-changed=wrapper.hpp");

//     let bindings = bindgen::Builder::default()
//         .header("wrapper.hpp")
//         // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .generate()
//         .expect("Unable to generate bindings");

//     let path = PathBuf::from("src");
//     bindings
//         .write_to_file(path.join("abieos_bindings.rs"))
//         .expect("Couldn't write bindings!");
// }
