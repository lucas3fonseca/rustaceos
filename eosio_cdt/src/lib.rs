#![allow(non_camel_case_types)]

mod eosio_cdt_bindings;
pub mod cdt;
pub mod eos;

pub use cdt::*;

pub type c_char = u8;
pub use core::ffi::c_void;

pub enum EosError {
    FailToGetActionInstance,
}
