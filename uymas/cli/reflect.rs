use proc_macro::TokenStream;

#[proc_macro_derive(CliApp)]
pub fn cli_app(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn cli_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn cli_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
