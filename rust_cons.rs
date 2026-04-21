use std::rc::Rc;

#[derive(Debug)]
enum ConsList<T> {
    Cons(T, Rc<ConsList<T>>),
    Nil,
}

use crate::ConsList::{Cons, Nil};

fn main() {
    let cons_list_1 =
        Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("cons_list_1 is {cons_list_1:?}");
    println!("cons_list_1 refcount {}", Rc::strong_count(&cons_list_1));

    let cons_list_2 = 
        Cons(0, Rc::clone(&cons_list_1));
    println!("cons_list_2 is {cons_list_2:?}");

    let Cons(v2, n2) = cons_list_2 else { panic!(); };
    println!("cons_list_1 refcount {}", Rc::strong_count(&n2));

    let cons_list_3 =
        Cons(-1, Rc::clone(&cons_list_1));
    println!("cons_list_3 is {cons_list_3:?}");
    let Cons(v3, n3) = cons_list_3 else { panic!(); };
    println!("cons_list_1 refcount {}", Rc::strong_count(&n3));
}
