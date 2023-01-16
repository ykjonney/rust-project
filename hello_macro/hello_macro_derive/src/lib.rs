mod builder;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    println!("{input:?}");
    let ast: DeriveInput = syn::parse(input).unwrap();
    println!("{ast:#?}");
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let context = builder::BuilderContext::new(input);
    let token = context.generate();
    token.into()
}
