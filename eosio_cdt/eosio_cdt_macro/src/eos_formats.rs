use proc_macro2::Ident;
use quote::format_ident;

pub fn format_action(ident: &Ident) -> Ident {
  format_ident!("EosAction{}", ident)
}

pub fn format_table(ident: &Ident) -> Ident {
  format_ident!("EosTable{}", ident)
}
