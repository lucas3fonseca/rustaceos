use eosio_cdt::eos;
use eosio_cdt::{print, require_auth, Action, Contract};

#[eosio_cdt::action]
fn hi(name: eos::Name) {
    // require_auth(contract.get_self());
    print!("Hi, ", name, "!");
}

#[eosio_cdt::action]
fn hello(owner: eos::Name) {
    require_auth(&owner);
    print!("hello ", owner);
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    let mut contract = Contract::new(eos::Name::new(receiver), eos::Name::new(code));

    if contract.is_contract_code() {
        let action_name = eos::Name::new(action);
        match action_name {
            EOS_hiAction::NAME => eosio_cdt::execute_action::<EOS_hiAction>(&mut contract),
            EOS_helloAction::NAME => eosio_cdt::execute_action::<EOS_helloAction>(&mut contract),
            _ => print!("Action not implemented"),
        }
    }
}
