#![allow(non_camel_case_types)]

use serde::de::DeserializeOwned;

pub mod eos;
mod eosio_cdt_bindings;

pub type c_char = u8;
pub use core::ffi::c_void;

pub enum EosError {
    FailToGetActionInstance,
}

pub struct Contract {
    contract: eos::Name,
    code: eos::Name,
    data_stream: Option<Vec<u8>>,
}

impl Contract {
    pub fn new(contract: eos::Name, code: eos::Name) -> Self {
        Contract {
            contract,
            code,
            data_stream: None,
        }
    }

    pub fn set_data_stream(&mut self, data_stream: Vec<u8>) {
        self.data_stream = Some(data_stream);
    }

    pub fn call_action(&self) -> bool {
        self.contract.value == self.code.value
    }

    pub fn get_self(&self) -> &eos::Name {
        &self.contract
    }
}

pub trait Action: DeserializeOwned {
    const NAME: eos::Name;
    fn execute(&self, contract: &Contract);
}

pub fn execute_action<T: Action>(contract: &mut Contract) {
    let byte_size = unsafe { eosio_cdt_bindings::action_data_size() };
    let mut bytes: Vec<u8> = vec![0; byte_size as usize];
    let buffer = &mut bytes[..] as *mut _ as *mut c_void;
    unsafe { eosio_cdt_bindings::read_action_data(buffer, byte_size) };

    let action_instance: T = bincode::deserialize(&bytes[..]).expect("fail to decode action data");
    contract.set_data_stream(bytes);
    action_instance.execute(contract);
}

pub fn require_auth(account: &eos::Name) {
    unsafe { eosio_cdt_bindings::require_auth(account.value) }
}

pub fn print(message: &str) {
    let ptr = message.as_ptr();
    let len = message.len() as u32;
    unsafe { eosio_cdt_bindings::prints_l(ptr, len) };
}
