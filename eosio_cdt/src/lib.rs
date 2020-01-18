#![allow(non_camel_case_types)]

pub mod cdt;
pub mod eos;
mod eosio_cdt_bindings;

pub use cdt::*;

pub type c_char = u8;
pub use core::ffi::c_void;

pub enum EosError {
    FailToGetActionInstance,
}
