extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn action(_args: TokenStream, input: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(args as AttributeArgs);
    // eprintln!("input tokens {:?}", input.to_token_stream());
    let input = parse_macro_input!(input as ItemFn);
    eprintln!("input {:?}", input);

    let ident = &input.sig.ident;
    let action_struct = format_ident!("EOS_{}Action", &ident.to_string());
    let action_fields: Vec<proc_macro2::TokenStream> = input.sig.inputs.iter()
        .map(|i| match i {
            syn::FnArg::Typed(arg) => quote_spanned!(arg.span()=> #arg),
            syn::FnArg::Receiver(rec) => quote_spanned!(rec.span()=>
                compile_error!("receiver argument (&self) is not allowed")),
        })
        .collect();

    let output = quote! {
        #[derive(eosio_cdt::eos::Serialize,
            eosio_cdt::eos::Deserialize)]
        pub struct #action_struct {
            #(#action_fields),*
        }

        impl eosio_cdt::Action for #action_struct {
            const NAME: eosio_cdt::eos::Name =
                eosio_cdt::eos::name_from_str(stringify!(#ident));

            fn execute(&self, contract: &eosio_cdt::Contract) {
                eosio_cdt::print!("wip action macro for contract ", *contract.get_self());
            }
        }
    };

    // let output = quote!{ #input.to_token_stream() };
    eprintln!("output tokens: {}", output);
    output.into()
}
