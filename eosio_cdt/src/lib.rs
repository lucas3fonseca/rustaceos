#![allow(non_camel_case_types)]

use bytes::Bytes;

pub mod eos;
mod eosio_cdt_bindings;

pub type c_char = u8;
pub use core::ffi::c_void;

pub enum EosError {
    FailToReadBytes,
}

pub struct Contract {
    contract: eos::Name,
    code: eos::Name,
    data_stream: Option<Bytes>,
}

impl Contract {
    pub fn new(contract: eos::Name, code: eos::Name) -> Self {
        Contract {
            contract,
            code,
            data_stream: None,
        }
    }

    pub fn set_data_stream(&mut self, data_stream: Bytes) {
        self.data_stream = Some(data_stream);
    }

    pub fn call_action(&self) -> bool {
        self.contract.value == self.code.value
    }

    pub fn get_self(&self) -> &eos::Name {
        &self.contract
    }
}

pub trait Action: Sized {
    const NAME: eos::Name;
    fn read_data(bytes: &mut Bytes) -> Result<Self, EosError>;
    fn execute(&self, contract: &Contract);
}

pub fn execute_action<T: Action>(contract: &mut Contract) -> Result<(), EosError> {
    let byte_size = unsafe { eosio_cdt_bindings::action_data_size() };
    let mut bytes: Vec<u8> = vec![0; byte_size as usize];
    let buffer = &mut bytes[..] as *mut _ as *mut c_void;
    unsafe { eosio_cdt_bindings::read_action_data(buffer, byte_size) };
    // let mut pos = 0;
    let mut bytes_buf = Bytes::from(bytes);
    let action_instance = T::read_data(&mut bytes_buf)?;
    contract.set_data_stream(bytes_buf);
    action_instance.execute(contract);
    Ok(())
}

pub fn require_auth(account: &eos::Name) {
    unsafe { eosio_cdt_bindings::require_auth(account.value) }
}

pub fn print(message: &str) {
    let ptr = message.as_ptr();
    let len = message.len() as u32;
    unsafe { eosio_cdt_bindings::prints_l(ptr, len) };
}
