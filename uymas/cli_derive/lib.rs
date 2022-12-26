use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// **注意** `derive` 属性尝试实现代码自动生成，但由于资料以及文档方面欠缺。其暂时为实验性的，不可用于生产。
/// 意识着代码还需更多的知识储备。
///

/// 定义命令行结构体
#[proc_macro_derive(CliApp)]
pub fn cli_app(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_run_macro(&ast)
}

// 生成 run
fn impl_run_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CliApp for #name {
            fn run(&self) {
                let mut cmd = crate::Cmd::new();
                // 默认方法
                cmd.empty(|_args: &crate::Args|{
                    println!("Hello, Macro! My name is {}!", stringify!(#name));
                });
                cmd.run();
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn cli_command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ipt_parse: syn::ItemFn = syn::parse(input.clone()).expect("Input sync parse error");
    let sig = ipt_parse.sig;
    let fn_name = sig.ident.clone().to_string();
    let name = &sig.ident;

    let attr_dbg = format!("{}", attr);
    let item_dbg = format!("{}", input);

    let args = attr.into_iter().count();
    let input = input.into_iter().count();
    let gen = quote! {
        pub fn #name(&self) {
            println!("entering");
            println!("attr_dbg: {}", #attr_dbg);
            println!("item_dbg: {}", #item_dbg);
            println!("fn_name: {}", #fn_name);
            println!("args tokens: {}", #args);
            println!("input tokens: {}", #input);
            println!("exiting");
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn cli_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {:?}", attr);
    println!("item: {:?}", item);
    TokenStream::new()
}
