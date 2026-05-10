mod print_hello_trait;
use crate::print_hello_trait::PrintHelloTrait;

use custom_derive_macro::ApplyPrintHelloTrait;

#[derive(ApplyPrintHelloTrait)]
struct MyStruct { }

fn main() {
    println!("Hello, world!");
    MyStruct::print_hello();
}
