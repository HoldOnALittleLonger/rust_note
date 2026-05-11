use std::cell::RefCell;
use std::string;

fn main() {
    let str_v = String::from("str_v");
    let ref_str = RefCell::new(str_v);
    println!("{}", ref_str.borrow());
    ref_str.borrow_mut().push_str(" modified");
    println!("{}", ref_str.borrow());

    let ref_str_copy = RefCell::clone(&ref_str);
    ref_str_copy.borrow_mut().push_str(" copy");
    println!("{}", ref_str.borrow());
    println!("{}", ref_str_copy.borrow());
}
