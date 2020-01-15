// use eosio_cdt::*;

// pub struct Name(u64);

// eosio_cdt::abi!(hi);

// #[eosio::action]
// fn hi(name: Name) {
//     eosio_cdt::print!("Hello, ", name, "!");
// }

// #[no_mangle]
// pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
//     if code == receiver {
//         match action {
//             eosio::Name::from("hi").value() => {
//                 // eosio::execute_action(
//                 //     eosio::Name::from(receiver),
//                 //     eosio::Name::from(code),
//                 // )
//                 let data = read_action_data::<hi>()
//                     .expect("failed to read action data");
//                 <hi as eosio::ActionFunction>::call(data);
//             }
//         }
//     }
// }
