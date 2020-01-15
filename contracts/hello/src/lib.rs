use eosio_cdt::{require_auth, print};
use eosio_cdt::eos;

fn hi(name: eos::Name) {
    require_auth(&name);
    let msg = format!("Hi, {}", name.value);
    print(msg.as_str());
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    if code == receiver {
        print("contract hello receiving action!");

        let action_hi = eos::Name::from("hi").unwrap();

        if action == action_hi.value {
            hi(eos::Name::from("hello").unwrap());
        } else {
            // todo: assert?
            print("Action not implemented");
        }
    }
}
