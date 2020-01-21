#[macro_export]
macro_rules! deprecated_eos_abi {
    ($($action:ty),*) => {
        #[no_mangle]
        pub extern "C" fn wip_apply(receiver: u64, code: u64, action: u64) {
            let mut contract = eosio_cdt::Contract::new(
                eos::Name::new(receiver),
                eos::Name::new(code),
            );

            if contract.is_contract_code() {
                match action {
                    $(
                        EOS_$actionAction::NAME => $crate::execute_action::<$action as $crate::Action>(&mut contract)
                    ),*
                    _ => print!("Action not implemented"),
                }
            }
        }
    };
}
