use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[eosio_cdt::action]
pub fn checkage(name: eos::Name, age: u16) {
    require_auth(&name);
    check(
        age >= 18,
        "OOpsss... Looks like you don't have the required age",
    );
    print!(
        "Hey ",
        name, ", looks like you have ", age, ".You are considered adult for our services."
    );
}
