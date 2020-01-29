extern crate proc_macro;

mod action;
mod dispatcher;
mod eos_formats;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    action::action(args, input)
}

#[proc_macro]
pub fn abi(input: TokenStream) -> TokenStream {
    dispatcher::abi(input)
}
