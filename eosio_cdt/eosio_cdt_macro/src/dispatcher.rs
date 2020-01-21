use crate::eos_formats::format_action;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

type ActionList = Punctuated<Ident, Token![,]>;

pub fn abi(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_input(input_tokens);
    eprintln!(
        "\n>> EOS Generating ABI Dispatcher for actions:\n{}",
        input.to_token_stream()
    );

    let actions_matchers = generate_actions_matchers(&input);
    let output = generate_contract_apply(&actions_matchers);

    eprintln!("\n>>> EOS Generated ABI Dispatcher: \n{}", output);
    proc_macro::TokenStream::from(output)
}

fn parse_input(input: proc_macro::TokenStream) -> ActionList {
    let parser = ActionList::parse_terminated;
    parser
        .parse(input)
        .expect("fail to parse eos_abi!, expects: `eos_abi!(action1, action2, ...);`")
}

fn generate_actions_matchers(actions: &ActionList) -> Vec<TokenStream> {
    actions
        .iter()
        .map(|action| {
            let action_ident = format_action(action);
            quote_spanned!(action.span()=> #action_ident::NAME =>
            eosio_cdt::execute_action::<#action_ident>(&mut contract))
        })
        .collect()
}

fn generate_contract_apply(actions_matchers: &Vec<TokenStream>) -> TokenStream {
    quote! {
        use eosio_cdt::cdt::action::Action;

        #[no_mangle]
        pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
            let mut contract = eosio_cdt::Contract::new(
              eos::Name::new(receiver),
              eos::Name::new(code)
            );

            if contract.is_contract_code() {
                let action_name = eos::Name::new(action);
                match action_name {
                    #(#actions_matchers),*,
                    _ => print!("Action not implemented"),
                }
            }
        }
    }
}
