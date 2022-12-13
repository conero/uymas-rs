extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn cli_app(input: TokenStream) {}

#[proc_macro_attribute]
pub fn cli_command(attr: TokenStream, item: TokenStream) {}

#[proc_macro_attribute]
pub fn cli_action(attr: TokenStream, item: TokenStream) {}
