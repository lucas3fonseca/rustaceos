#![feature(const_fn, const_if_match, const_loop, const_panic, const_mut_refs)]
#![allow(non_camel_case_types)]

pub use core::ffi::c_void;

pub mod cdt;
pub mod eos;
mod eosio_cdt_bindings;

pub use cdt::*;
pub use eosio_cdt_macro::{abi, action};

pub type c_char = u8;
