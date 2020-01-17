use bytes::Bytes;
use eosio_cdt::eos;
use eosio_cdt::eos::AbiRead;
use eosio_cdt::{print, require_auth, Action, Contract, EosError};

pub struct HiAction {
    name: eos::Name,
}

impl Action for HiAction {
    const NAME: eos::Name = eos::Name {
        value: 7746191359077253120u64,
    }; //eos::Name::from("hi").unwrap();

    fn read_data(bytes: &mut Bytes) -> Result<Self, EosError> {
        let name = eos::Name::read(bytes);
        Ok(HiAction { name })
    }

    fn execute(&self, contract: &Contract) {
        require_auth(contract.get_self());
        let msg = format!("Hi, {}!", self.name.value);
        print(msg.as_str());
    }
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    let mut contract = Contract::new(eos::Name::new(receiver), eos::Name::new(code));

    if contract.call_action() {
        print("contract calling action");

        if action == HiAction::NAME.value {
            eosio_cdt::execute_action::<HiAction>(&mut contract);
        } else {
            print("Action not implemented");
        }
    }
}
