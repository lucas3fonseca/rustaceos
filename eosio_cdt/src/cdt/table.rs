use core::ptr::null_mut;
use serde::Serialize;

use serde::de::DeserializeOwned;

use crate::c_void;
use crate::check;
use crate::eos;
use crate::eosio_cdt_bindings;
use crate::expect;

pub struct TableIterator {
    code: eos::Name,
    scope: eos::Name,
    name: eos::Name,
    itr: Option<i32>,
}

pub trait Table {
    fn new(code: eos::Name, scope: eos::Name) -> TableIterator;
    fn pk(&self) -> u64;
}

impl TableIterator {
    pub fn new(code: eos::Name, scope: eos::Name, name: eos::Name) -> TableIterator {
        TableIterator {
            code,
            scope,
            name,
            itr: None,
        }
    }

    pub fn insert<T>(&self, payer: &eos::Name, data: &T)
    where
        T: Serialize + Table,
    {
        table_insert(&self.scope, &self.name, payer, data.pk(), data);
    }

    pub fn find(&mut self, id: u64) -> i32 {
        let itr = table_find(&self.code, &self.scope, &self.name, id);
        self.itr = Some(itr);
        itr
    }

    pub fn begin(&mut self) -> Option<i32> {
        let itr = table_lower_bound(&self.code, &self.scope, &self.name, 0);
        if itr != self.end() {
            self.itr = Some(itr);
            Some(itr)
        } else {
            None
        }
    }

    pub fn end(&self) -> i32 {
        table_end(&self.code, &self.scope, &self.name)
    }

    pub fn read<T: DeserializeOwned>(&self) -> T {
        let itr = expect(self.itr, "invalid record");
        table_get(itr)
    }

    pub fn erase(&mut self) {
        let itr = expect(self.itr, "iterator is not valid to delete");
        table_remove(itr);
    }

    pub fn get<T: DeserializeOwned>(&mut self, id: u64) -> T {
        let itr = self.find(id);
        check(itr != self.end(), "invalid record");
        table_get(itr)
    }

    pub fn update<T: Serialize>(&self, payer: &eos::Name, data: &T) {
        let itr = expect(self.itr, "iterator is not valid to update");
        table_update(payer, itr, data);
    }

    pub fn delete(&mut self, id: u64) {
        let itr = self.find(id);
        check(itr != self.end(), "invalid record to delete");
        self.erase();
    }
}

impl Iterator for TableIterator {
    type Item = TableIterator;

    fn next(&mut self) -> Option<Self::Item> {
        let itr = expect(self.itr, "cannot advance empty iterator");
        let next_itr = table_next_i64(itr).0;
        if itr >= 0 {
            self.itr = Some(next_itr);
            let item = TableIterator {
                code: self.code,
                scope: self.scope,
                name: self.name,
                itr: Some(itr),
            };
            Some(item)
        } else {
            self.itr = None;
            None
        }
    }
}

pub fn table_find(code: &eos::Name, scope: &eos::Name, table: &eos::Name, id: u64) -> i32 {
    unsafe { eosio_cdt_bindings::db_find_i64(code.value, scope.value, table.value, id) }
}

pub fn table_next_i64(itr: i32) -> (i32, u64) {
    let mut id: u64 = 0;
    let pk: *mut u64 = &mut id;
    let next_itr = unsafe { eosio_cdt_bindings::db_next_i64(itr, pk) };
    (next_itr, id)
}

pub fn table_lower_bound(code: &eos::Name, scope: &eos::Name, table: &eos::Name, id: u64) -> i32 {
    unsafe { eosio_cdt_bindings::db_lowerbound_i64(code.value, scope.value, table.value, id) }
}

pub fn table_end(code: &eos::Name, scope: &eos::Name, table: &eos::Name) -> i32 {
    unsafe { eosio_cdt_bindings::db_end_i64(code.value, scope.value, table.value) }
}

pub fn table_get<T: DeserializeOwned>(itr: i32) -> T {
    let buffer: *mut c_void = null_mut();
    let byte_size = unsafe { eosio_cdt_bindings::db_get_i64(itr, buffer, 0) } as u32;

    let mut bytes: Vec<u8> = vec![0; byte_size as usize];
    let buffer = bytes.as_mut_ptr().cast();

    unsafe {
        eosio_cdt_bindings::db_get_i64(itr, buffer, byte_size);
    }
    bincode::deserialize(&bytes[..]).expect("fail to decode table data")
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

pub fn table_update<T: ?Sized>(payer: &eos::Name, iterator: i32, data: &T)
where
    T: Serialize,
{
    let mut bytes = bincode::serialize(data).expect("fail to encode table data");
    let byte_size = bytes.len() as u32;
    let buffer = bytes.as_mut_ptr().cast();

    unsafe {
        eosio_cdt_bindings::db_update_i64(iterator, payer.value, buffer, byte_size);
    }

    // TODO: insert secondary keys
}

pub fn table_remove(itr: i32) {
    check(itr >= 0, "invalid iterator to remove");
    unsafe { eosio_cdt_bindings::db_remove_i64(itr) };
    // TODO: remove secondary keys too
}
