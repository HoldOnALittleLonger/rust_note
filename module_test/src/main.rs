mod module_x;
use crate::module_x::module_y::func;

fn main() {
    func();
    println!("Hello, world!");
}
