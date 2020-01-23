use serde::Serialize;
use core::ptr::null_mut;

use serde::de::DeserializeOwned;

use crate::c_void;
use crate::eos;
use crate::check;
use crate::eosio_cdt_bindings;

pub fn table_find(code: &eos::Name, scope: &eos::Name, table: &eos::Name, id: u64) -> i32 {
    unsafe { eosio_cdt_bindings::db_find_i64(code.value, scope.value, table.value, id) }
}

pub fn table_get<T: DeserializeOwned>(itr: i32) -> T {
    let buffer: *mut c_void = null_mut();
    let byte_size = unsafe { eosio_cdt_bindings::db_get_i64(itr, buffer, 0) } as u32;

    let mut bytes: Vec<u8> = vec![0; byte_size as usize];
    let buffer = bytes.as_mut_ptr().cast();

    unsafe { eosio_cdt_bindings::db_get_i64(itr, buffer, byte_size); }
    bincode::deserialize(&bytes[..]).expect("fail to decode table data")
}

pub fn table_remove(itr: i32) {
    check(itr >= 0, "invalid iterator to remove");
    unsafe { eosio_cdt_bindings::db_remove_i64(itr) };
}

pub fn table_end(code: &eos::Name, scope: &eos::Name, table: &eos::Name) -> i32 {
    unsafe { eosio_cdt_bindings::db_end_i64(code.value, scope.value, table.value) }
}

pub fn table_insert<T: ?Sized>(
    scope: &eos::Name,
    table: &eos::Name,
    payer: &eos::Name,
    id: u64,
    data: &T,
) -> i32
where
    T: Serialize,
{
    let mut bytes = bincode::serialize(data).expect("fail to encode table data");
    let byte_size = bytes.len() as u32;
    let buffer = bytes.as_mut_ptr().cast();

    let itr;
    unsafe {
        itr = eosio_cdt_bindings::db_store_i64(
            scope.value,
            table.value,
            payer.value,
            id,
            buffer,
            byte_size,
        );
    }

    // TODO: insert secondary keys

    itr
}

pub fn table_update<T: ?Sized>(
    payer: &eos::Name,
    iterator: i32,
    data: &T,
)
where
    T: Serialize,
{
    let mut bytes = bincode::serialize(data).expect("fail to encode table data");
    let byte_size = bytes.len() as u32;
    let buffer = bytes.as_mut_ptr().cast();

    unsafe { eosio_cdt_bindings::db_update_i64(iterator, payer.value, buffer, byte_size); }

    // TODO: insert secondary keys
}
