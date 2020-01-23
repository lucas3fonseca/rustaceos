use crate::eosio_cdt_bindings;

pub fn check(test: bool, message: &str) {
    if !test {
        let ptr = message.as_ptr();
        let len = message.len() as u32;
        unsafe { eosio_cdt_bindings::eosio_assert_message(0, ptr, len) };
    }
}
