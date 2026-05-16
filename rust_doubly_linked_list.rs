/// This file contains a demo implementation of doubly-linked ring list.
/// We do not define all the list container operations to simplify
/// the detail.
/// The definition to Node descriptor is very ugly,because we have
/// to prevent memory leak and have to distinguish head node and normal
/// node to determine where to stop the print traversing.
/// Unfortunately,the type std::rc::Weak<> is not implement PartialEq,
/// thus we can not to traverse the list like the Kernel Style.
///     next != head
/// When we test operator== on RefCell<>,it requires all fields of
/// its inner the types need to implement PartialEq.
/// Maybe we can wrap the Weak<> type into anothe type,and implement
/// PartialEq on the wrapper to achieve this indirectly,but that will
/// improve the dirty works we have to do.
/// Hence,just put a head indicator @is_head in node is more easly
/// to implement.
/// Or,implement operator== for Node<>,and do special handling on it,
/// such upgrade() the Weak<> object inside it.

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/*
 * PrevNodeConnector<> - type alias for a connector to previous node
 * # the reason use Weak<> to wrap RefCell<> is to prevent
 *   reference cycle.
 */
type PrevNodeConnector<_Tp> = Option<Weak<RefCell<Node<_Tp>>>>;
type NextNodeConnector<_Tp> = Option<Rc<RefCell<Node<_Tp>>>>;

/*
 * Node<> - list node structure
 * @item:   item
 * @is_head:
 *          mark list head to prevent infinity loop
 * @prev:   a wrapped Weak<> object to previous node,
 * @next:   a wrapped Rc<> object to next node
 * @tail_node_next:
 *          a wrapped Weak<> object to the head node,if
 *          current node is the tail
 */
struct Node<_Tp> {
    item: _Tp,
    is_head: bool,
    prev: PrevNodeConnector<_Tp>,
    next: NextNodeConnector<_Tp>,
    tail_node_next: PrevNodeConnector<_Tp>,
}
impl<_Tp> Node<_Tp> {
    fn new(value: _Tp) -> Self {
        Self {
            item: value,
            is_head: false,
            prev: None,
            next: None,
            tail_node_next: None,
        }
    }
}
impl<_Tp> Drop for Node<_Tp> {
    fn drop(&mut self) {
        println!("node drop is called.");
    }
}

macro_rules! prefetch_next_item {
    ( $rc_clone:expr ) => {
        &$rc_clone.borrow().item
    }
}

/* DList<> - dummy head */
struct DList<_Tp: std::fmt::Display> {
    head: NextNodeConnector<_Tp>,
}
impl<_Tp: std::fmt::Display> Drop for DList<_Tp> {
    fn drop(&mut self) {
        println!("The DList drop() is called.");
    }
}

impl<_Tp: std::fmt::Display> DList<_Tp> {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    /*
     * prefetch routine can be used by FIND primitive,and which will can be
     * used for DELETE primitive.
     * The problem is,when we decompose and Option<>,if we do not return
     * ownership,then a temporary object would be created and bound to the
     * inner.
     * We can not return a reference to that temporary object directly,
     * because a function should not return a reference to a stack object.
     * Thus,we need to copy the @item.
     * Another way is,return Rc<> to the next node instead return @item.
     * In this case,the logic of prefetch_next_item() needed to be moved into
     * where prefetch is required,no longer encapsolated into a function.
     * For simplify the works,we can use a macro prefetch_next_item!() macro
     * to do this.No longer need Clone trait.
     */
    fn __prefetch_next(node: &Node<_Tp>) -> NextNodeConnector<_Tp> {
        if let None = node.next {
            None
        } else {
            let next_node_rc_clone = Rc::clone(node.next.as_ref().unwrap());
            Some(next_node_rc_clone)
        }
    }

    /* just for test */
    fn retrieve_next_item(node: &Node<_Tp>) {
        if let Some(rc_clone) = Self::__prefetch_next(node) {
            // let _:() = prefetch_next_item!(rc_clone); => &_Tp
            println!("item is {}", prefetch_next_item!(rc_clone));
        }
    }

    fn dlist_add(&mut self, value: _Tp) {
        let new_node_ref = Rc::new(RefCell::new(
            Node::<_Tp>::new(value)
        ));

        /* initial state */
        if let None = self.head {
            new_node_ref.borrow_mut().is_head = true;
            self.head = Some(new_node_ref);
            return;
        }

        if let Some(current_node_ref) = self.head.take() {
            let mut only_two_elements = false;

            /* only head node now */
            if let None = current_node_ref.borrow().next.as_ref() {
                if let None = current_node_ref.borrow().prev.as_ref() {
                    only_two_elements = true;
                }
            }

            /*
             * for the case that size of this list is 2,including the
             * new node it is preparing to be inserted.
             *    None <= [head] => None
             *                   ^^
             *                   || insert
             *          None <= [new node] => None
             *        
             *        +--------------------------+
             *        |  +---------------+       |
             *        |  |               | prev  |
             *        V  V  next         |       |
             *    +-- [head] --> [new node] -----+ next
             *    |              ^
             *    |              | prev
             *    +--------------+
             */
            if only_two_elements {
                /*
                 * we always insert the new node right after head node,
                 * thus the second inserted node will becomes the tail node.
                 * through this Weak<> object,we can reduce the head node's
                 * strong count,then no memory leak will happens.
                 */
                new_node_ref.borrow_mut().tail_node_next = Some(Rc::downgrade(&current_node_ref));
                new_node_ref.borrow_mut().prev = Some(Rc::downgrade(&current_node_ref));
                current_node_ref.borrow_mut().prev = Some(Rc::downgrade(&new_node_ref));
                current_node_ref.borrow_mut().next = Some(new_node_ref.clone());
                self.head = Some(current_node_ref);
                return;
            }

            /*
             * for the case that size of this list is greater than 2 currently,
             * including the  new node it is preparing to be inserted
             *          +--------------+
             *          |              | next
             *          V next         |
             *     [head] <=> [old next]
             *     |      prev ^  ^^
             *     |           |  || insert at front of old next
             *     +-----------+  ||
             *         prev       [new node]
             *
             *          +---------------------------------+
             *          |                                 |
             *          V                                 |
             *     [head] <=> [new node] <=> [old next] --+
             *     |                         ^
             *     |                         |
             *     +-------------------------+
             */
            let next_node_ref = current_node_ref.borrow_mut().next.take().unwrap();
            new_node_ref.borrow_mut().next = Some(next_node_ref.clone());
            new_node_ref.borrow_mut().prev = Some(Rc::downgrade(&current_node_ref));
            next_node_ref.borrow_mut().prev = Some(Rc::downgrade(&new_node_ref));
            current_node_ref.borrow_mut().next = Some(new_node_ref.clone());
            self.head = Some(current_node_ref);
        }
    }

    fn __dlist_print_prev(weakref: &Weak<RefCell<Node<_Tp>>>) {
        let weak_clone = weakref.clone();
        let rcref = weak_clone.upgrade().unwrap();

        if rcref.borrow().is_head {
            println!("reached head,return.");
            return;
        } else {
            println!("node strong count is {}", Rc::strong_count(&rcref));
            println!("node weak count is {}", Rc::weak_count(&rcref));
            println!("item : {}", rcref.borrow().item);
            Self::__dlist_print_prev(rcref.borrow().prev.as_ref().unwrap());
        }
    }

    fn dlist_print_prev(&self) {
        if let None = self.head.as_ref() {
            return;
        } else {
            println!("head strong count is {}", Rc::strong_count(&self.head.as_ref().unwrap()));
            println!("item : {}", self.head.as_ref().unwrap().borrow().item);
            Self::__dlist_print_prev(self.head.as_ref().unwrap().borrow().prev.as_ref().unwrap());
        }
    }

    /*
     * __dlist_print - cross function to compare Rc<> reference for determine
     *                 where to stop recursion is not work,thus we mark the
     *                 head node with @is_head == true.
     */
    fn __dlist_print(rcref: &Rc<RefCell<Node<_Tp>>>) {
        if rcref.borrow().is_head {
            return;
        } else {
            println!("node strong count is {}", Rc::strong_count(rcref));
            println!("item : {}", rcref.borrow().item);
            if let None = rcref.borrow().next {
                match rcref.borrow().tail_node_next.as_ref() {
                    Some(n) => {
                        let weak_clone = n.clone();
                        let next_ref = weak_clone.upgrade().unwrap();
                        Self::__dlist_print(&next_ref);
                    },
                    _ => return,
                }
            } else {
                match rcref.borrow().next.as_ref() {
                    Some(n) => Self::__dlist_print(n),
                    _ => return,
                }
            }


        }
    }

    fn dlist_print(&self) {
        match self.head.as_ref() {
            Some(v) => {
                println!("head strong count is {}", Rc::strong_count(v));
                println!("item : {}", v.borrow().item);
                match v.borrow().next.as_ref() {
                    Some(n) => Self::__dlist_print(n),
                     _ => return,
                };
            },
            _ => return,
        }
    }

}


fn main() {
    let int_v = 32;
    let rc_ref = Rc::new(RefCell::new(int_v));
    let rc_ref_clone = rc_ref.clone();
    if rc_ref == rc_ref_clone {
        println!("same object");
    }
    println!("------------------------");

    let mut dlist = DList::<i32>::new();
    dlist.dlist_add(0);

    if let Some(v) = dlist.head.as_ref() {
        println!("@dlist->head strong count is {}",
                 Rc::strong_count(v));
    }
    println!("------------------------");

    dlist.dlist_add(1);
    dlist.dlist_add(2);
    dlist.dlist_add(3);
    dlist.dlist_add(4);
    dlist.dlist_add(5);
    dlist.dlist_add(6);
    dlist.dlist_add(7);
    dlist.dlist_add(8);
    dlist.dlist_add(9);

    DList::<i32>::retrieve_next_item(&*dlist.head.as_ref().unwrap().borrow());
    println!("------------------------");

    dlist.dlist_print();
    println!("------------------------");
    dlist.dlist_print_prev();
}
