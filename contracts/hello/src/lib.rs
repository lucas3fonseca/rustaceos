use eosio_cdt;
use eosio_cdt::eos;
use eosio_cdt::{print, require_auth};

mod age;
use age::*;

eosio_cdt::abi!(hello, signupage);

#[eosio_cdt::action]
pub fn hello(owner: eos::Name) {
    require_auth(&owner);
    print!("Welcome ", owner);
}
