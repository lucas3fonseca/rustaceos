extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let clang_args = vec![
        "-I", "./external/eosio.cdt/libraries/libc/musl/include",
        "-I", "./external/eosio.cdt/libraries",
    ];
    let builder = bindgen::Builder::default()
        .header("wrapper.hpp")
        .whitelist_function("require_auth")
        .whitelist_function("prints_l")
        .ctypes_prefix("crate")
        .trust_clang_mangling(false)
        .clang_args(clang_args);

    print!(">>> Generating bindings executing:\nbindgen");
    for param in builder.command_line_flags() {
        print!(" {}", param);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let path = PathBuf::from("src");
    bindings
        .write_to_file(path.join("eosio_cdt_bindings.rs"))
        .expect("Couldn't write bindings!");
}
