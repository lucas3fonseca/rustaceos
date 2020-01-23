extern crate proc_macro;

mod eos_formats;
mod action;
mod dispatcher;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    action::action(args, input)
}

#[proc_macro]
pub fn abi(input: TokenStream) -> TokenStream {
    dispatcher::abi(input)
}
