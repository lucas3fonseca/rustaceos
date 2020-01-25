use crate::eos_formats::format_action;
use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, parse_macro_input, ItemFn};

pub fn action(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let ident = &input.sig.ident;
    let action_struct = format_action(ident);

    let mut action_fields = Vec::new();
    let mut fields_setup = Vec::new();

    for input_field in &input.sig.inputs {
        match input_field {
            syn::FnArg::Typed(arg) => {
                let field = &arg.pat;
                let ty = &arg.ty;
                action_fields.push(quote_spanned!(field.span()=> #field : #ty));
                fields_setup.push(quote_spanned!(ty.span()=> let #field = self.#field));
            }
            _ => action_fields.push(quote_spanned!(input_field.span()=>
                compile_error!("receiver argument (&self) is not allowed"))),
        }
    }

    let action_block = &input.block;

    let output = quote! {
        #[derive(eosio_cdt::eos::Deserialize)]
        pub struct #action_struct {
            #(#action_fields),*
        }

        impl eosio_cdt::Action for #action_struct {
            const NAME: eosio_cdt::eos::Name =
                eosio_cdt::eos::name_from_str(stringify!(#ident));

            fn execute(self, contract: &eosio_cdt::Contract) {
                #(#fields_setup);*;
                #action_block
            }
        }
    };

    output.into()
}
