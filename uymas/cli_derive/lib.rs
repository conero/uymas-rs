use proc_macro::TokenStream;
use quote::quote;
use syn;

/// 定义命令行结构体
#[proc_macro_derive(CliApp)]
pub fn cli_app(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_run_macro(&ast)
}

// 生成 run
fn impl_run_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CliApp for #name {
            fn run(&self) {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn cli_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_dbg = format!("{}", attr)
        .replace("\"", "\\\"")
        .replace("{", "{{")
        .replace("}", "}}");
    let item_dbg = format!("{}", item)
        .replace("\"", "\\\"")
        .replace("{", "{{")
        .replace("}", "}}");
    let x = format!(
        r#"
        pub fn dummy() {{
            println!("entering");
            println!("{attr_dbg}");
            println!("{item_dbg}");
            println!("args tokens: {{}}", {args});
            println!("input tokens: {{}}", {input});
            println!("exiting");
        }}
    "#,
        args = attr.into_iter().count(),
        input = item.into_iter().count(),
    );

    x.parse().expect("Generated invalid tokens")
}

#[proc_macro_attribute]
pub fn cli_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {:?}", attr);
    println!("item: {:?}", item);
    TokenStream::new()
}
