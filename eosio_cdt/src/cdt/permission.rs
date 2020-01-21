use crate::eos;
use crate::eosio_cdt_bindings;

pub fn require_auth(account: &eos::Name) {
    unsafe { eosio_cdt_bindings::require_auth(account.value) }
}
