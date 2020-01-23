use serde::Serialize;

use crate::eos;
use crate::eosio_cdt_bindings;

pub fn table_find(code: &eos::Name, scope: &eos::Name, table: &eos::Name, id: u64) -> i32 {
    unsafe { eosio_cdt_bindings::db_find_i64(code.value, scope.value, table.value, id) }
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
    let buffer = bytes.as_mut_ptr().cast();

    let itr;
    unsafe {
        itr = eosio_cdt_bindings::db_store_i64(
            scope.value,
            table.value,
            payer.value,
            id,
            buffer,
            bytes.len() as u32,
        );
    }

    // TODO: insert secondary keys

    itr
}