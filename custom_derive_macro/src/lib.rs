/// This file contains the procedural macro PrintHello.
/// The macro will applies trait PrintHelloTrait to the
/// struct type or enum type that #[derive] the macro.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ApplyPrintHelloTrait)]
pub fn apply_print_hello_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_apply_print_hello_trait(&ast)
}

fn impl_apply_print_hello_trait(ast: &syn::DeriveInput) -> TokenStream {
    let idt = &ast.ident;
    let output = quote! {
        impl PrintHelloTrait for #idt {
            fn print_hello() {
                println!("hello {}", stringify!(#idt));
            }
        }
    };
    output.into()
}

