use proc_macro2::Ident;
use quote::format_ident;

pub fn format_action(ident: &Ident) -> Ident {
  format_ident!("EOS_{}Action", ident)
}
