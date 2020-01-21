extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn action(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    // eprintln!("input {:?}", input);

    let ident = &input.sig.ident;
    let action_struct = format_ident!("EOS_{}Action", &ident.to_string());

    let mut action_fields = Vec::new();
    let mut fields_names = Vec::new();

    for input_field in &input.sig.inputs {
        match input_field {
            syn::FnArg::Typed(arg) => {
                let pat_val = arg.pat.as_ref();
                if let syn::Pat::Ident(name) = pat_val {
                    let name = &name.ident;
                    fields_names.push(quote_spanned!(arg.span()=> self.#name));

                    let pat_ty = arg.ty.as_ref();
                    action_fields.push(quote_spanned!(arg.span()=> #name : #pat_ty));
                }
            },
            syn::FnArg::Receiver(rec) =>
                action_fields.push(quote_spanned!(rec.span()=>
                compile_error!("receiver argument (&self) is not allowed"))),
        }
    }

    let output = quote! {
        #input

        #[derive(eosio_cdt::eos::Serialize,
            eosio_cdt::eos::Deserialize)]
        pub struct #action_struct {
            #(#action_fields),*
        }

        impl eosio_cdt::Action for #action_struct {
            const NAME: eosio_cdt::eos::Name =
                eosio_cdt::eos::name_from_str(stringify!(#ident));

            fn execute(self, contract: &eosio_cdt::Contract) {
                #ident(#(#fields_names),*);
            }
        }
    };

    // let output = quote!{ #input.to_token_stream() };
    eprintln!("\n>>> Output tokens: {}", output);
    output.into()
}
