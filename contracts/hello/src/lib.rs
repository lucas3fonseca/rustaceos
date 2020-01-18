use eosio_cdt::eos;
use eosio_cdt::eos::{Deserialize, Serialize};
use eosio_cdt::{print, require_auth, Action, Contract};

#[derive(Serialize, Deserialize)]
pub struct HiAction {
    name: eos::Name,
}

impl Action for HiAction {
    const NAME: eos::Name = eos::Name {
        value: 7746191359077253120u64,
    }; //eos::Name::from("hi").unwrap();

    fn execute(&self, contract: &Contract) {
        require_auth(contract.get_self());
        print!("Hi, ", self.name, "!");
    }
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    let mut contract = Contract::new(eos::Name::new(receiver), eos::Name::new(code));

    if contract.call_action() {
        if action == HiAction::NAME.value {
            eosio_cdt::execute_action::<HiAction>(&mut contract);
        } else {
            print!("Action not implemented");
        }
    }
}
