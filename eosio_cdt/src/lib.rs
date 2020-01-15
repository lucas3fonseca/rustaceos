mod eosio_cdt_bindings;
pub mod eos;

pub fn require_auth(account: eos::Name) {
  unsafe { eosio_cdt_bindings::require_auth(account.value) }
}

pub fn print(message: &str) {
  let ptr = message.as_ptr();
  let len = message.len() as u32;
  unsafe { eosio_cdt_bindings::prints_l(ptr as *const i8, len) };
}
