use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let rc = Rc::new(2);
    let weak_rc = Rc::downgrade(&rc);
    println!("rc {rc:?}");
    println!("weak_rc {weak_rc:?}");
    println!("rc strong count {}", Rc::strong_count(&rc));
    println!("rc weak count {}", Rc::weak_count(&rc));

    let upgrade_rc = weak_rc.upgrade();
    println!("upgrade_rc {upgrade_rc:?}");
    println!("rc strong count {}", Rc::strong_count(&rc));
    println!("rc weak count {}", Rc::weak_count(&rc));
}
