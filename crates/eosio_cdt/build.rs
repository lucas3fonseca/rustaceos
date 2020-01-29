extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let eoscdt_bindings_var = env::var("EOSCDT_BINDINGS");
    if let Ok(_) = eoscdt_bindings_var {
        generate_eoscdt_bindings();
    }
}

fn generate_eoscdt_bindings() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let clang_args = vec![
        "-I",
        "./external/eosio.cdt/libraries/libc/musl/include",
        "-I",
        "./external/eosio.cdt/libraries",
    ];
    let builder = bindgen::Builder::default()
        .header("wrapper.hpp")
        .whitelist_function("require_auth")
        .whitelist_function("read_action_data")
        .whitelist_function("action_data_size")
        .whitelist_function("has_auth")
        .whitelist_function("require_auth2")
        .whitelist_function("is_account")
        .whitelist_function("send_inline")
        .whitelist_function("send_context_free_inline")
        .whitelist_function("prints_l")
        .whitelist_function("printn")
        .whitelist_function("printi")
        .whitelist_function("printui")
        .whitelist_function("eosio_assert_message")
        .whitelist_function("db_store_i64")
        .whitelist_function("db_update_i64")
        .whitelist_function("db_remove_i64")
        .whitelist_function("db_get_i64")
        .whitelist_function("db_next_i64")
        .whitelist_function("db_previous_i64")
        .whitelist_function("db_find_i64")
        .whitelist_function("db_lowerbound_i64")
        .whitelist_function("db_upperbound_i64")
        .whitelist_function("db_end_i64")
        .whitelist_function("db_idx64_store")
        .whitelist_function("db_idx64_update")
        .whitelist_function("db_idx64_remove")
        .whitelist_function("db_idx64_next")
        .whitelist_function("db_idx64_previous")
        .whitelist_function("db_idx64_find_primary")
        .whitelist_function("db_idx64_find_secondary")
        .whitelist_function("db_idx64_lowerbound")
        .whitelist_function("db_idx64_upperbound")
        .whitelist_function("db_idx64_end")
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
