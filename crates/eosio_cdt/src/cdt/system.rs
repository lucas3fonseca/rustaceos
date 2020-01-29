use crate::eosio_cdt_bindings;
use std::process;

pub fn check(test: bool, message: &str) {
    if !test {
        let ptr = message.as_ptr();
        let len = message.len() as u32;
        unsafe { eosio_cdt_bindings::eosio_assert_message(0, ptr, len) };
    }
}

pub fn expect<T>(o: Option<T>, message: &str) -> T {
    match o {
        Some(t) => t,
        None => {
            check(false, message);
            process::abort()
        }
    }
}
