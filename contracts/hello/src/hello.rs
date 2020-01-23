use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[eosio_cdt::action]
pub fn hi(name: eos::Name) {
    require_auth(contract.get_self());
    print!("Hi, ", name, "!");
}

#[eosio_cdt::action]
pub fn hello(owner: eos::Name) {
    require_auth(&owner);
    print!("hello ", owner);
}
