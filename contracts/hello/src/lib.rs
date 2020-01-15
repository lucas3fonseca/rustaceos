use eosio_cdt::*;

pub struct Name(u64);

eosio_cdt::abi!(hi);

#[eosio::action]
fn hi(name: Name) {
    eosio_cdt::print!("Hello, ", name, "!");
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    if code == receiver {
        match action {
            eosio::Name::from("hi").value() => {
                // eosio::execute_action(
                //     eosio::Name::from(receiver),
                //     eosio::Name::from(code),
                // )
                let data = read_action_data::<hi>()
                    .expect("failed to read action data");
                <hi as eosio::ActionFn>::call(data);
            }
        }
    }

    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let message = payload
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().map(|s| s.to_string()))
            .unwrap_or_else(|| panic_info.to_string());
        $crate::check(false, &message);
    }));
    if code == eosio::n!("eosio") && action == eosio::n!("onerror") {
        panic!(
            "onerror action's are only valid from the \"eosio\" system account"
        )
    }

    $(
        else if code == receiver && action == <$t as eosio::ActionFn>::NAME.as_u64() {
            let data = $crate::read_action_data::<$t>().expect("failed to read action data");
            <$t as eosio::ActionFn>::call(data)
        }
    )+
    else {
        panic!("unknown action '{}'", eosio::Name::new(action));
    }
}
