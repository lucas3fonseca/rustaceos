use crate::eosio_cdt_bindings;
use crate::eos;

pub fn require_auth(account: &eos::Name) {
  unsafe { eosio_cdt_bindings::require_auth(account.value) }
}
