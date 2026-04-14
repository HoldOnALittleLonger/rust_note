mod module_x;
use crate::module_x::module_y::func;

mod module_z;
use crate::module_z::module_z::func_z;

mod fname_as_module;
use crate::fname_as_module::fname_as_module_func;

fn main() {
    func();
    func_z();
    fname_as_module_func();
    println!("Hello, world!");
}
