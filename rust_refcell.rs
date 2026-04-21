use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let share_ownership: Rc<RefCell<i32>> = 
        Rc::new(RefCell::new(20));
    let mut modifier1 = (*share_ownership).borrow_mut();
    println!("1 => {}", *modifier1);
    *modifier1 = 30;
    println!("1 => {}", *modifier1);
}
