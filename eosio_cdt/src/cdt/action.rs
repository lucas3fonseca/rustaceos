use serde::de::DeserializeOwned;

use super::contract::Contract;
use crate::eos;
use crate::eosio_cdt_bindings;

pub trait Action: DeserializeOwned {
    const NAME: eos::Name;
    fn execute(&self, contract: &Contract);
}

pub fn execute_action<T: Action>(contract: &mut Contract) {
    let byte_size = unsafe { eosio_cdt_bindings::action_data_size() };

    let mut bytes: Vec<u8> = vec![0; byte_size as usize];
    let buffer = bytes.as_mut_ptr().cast();
    unsafe { eosio_cdt_bindings::read_action_data(buffer, byte_size) };

    let action_instance: T = bincode::deserialize(&bytes[..]).expect("fail to decode action data");
    contract.set_data_stream(bytes);
    action_instance.execute(contract);
}
