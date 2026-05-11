/// This file contains the simple implementation of single-linked
/// list in Rust.

use std::cell::RefCell;

type ListNodeConnectorType<T> = RefCell<Option<Box<ListNode<T>>>>;

struct ListNode<T> {
    item: T,
    next: ListNodeConnectorType<T>,
}
impl<T: std::fmt::Display> ListNode<T> {

    /*
     * print_items - recursively print all items.
     */
    fn print_items(&self) {
        println!("{}", self.item);
        match *self.next.borrow() {
            Some(ref v) => {
                v.print_items();
            },
            None => return,
        }
    }
}

struct ListHead<T> {
    next: ListNodeConnectorType<T>,
}

impl<T: std::fmt::Display> ListHead<T> {
    fn new() -> Self {
        Self {
            next: RefCell::new(None),
        }
    }

    fn list_add(&mut self, value: T) {
        let node = ListNode::<T> {
            item: value,
            next: RefCell::new(self.next.take()),
        };
        self.next = RefCell::new(Some(Box::new(node)));
    }

    fn print_all(&self) {
        match self.next.borrow().as_ref() {
            Some(v) => v.print_items(),
            None => return,
        }
    }
}

fn print_nodes<T: std::fmt::Display>(node: &Box<ListNode<T>>) {
    println!("item : {}", node.item);
    match node.next.borrow().as_ref() {
        Some(v) => print_nodes(v),
        None => return,
    }
}

fn print_head<T: std::fmt::Display>(head: &ListHead<T>) {
    match *head.next.borrow() {
        Some(ref v) => print_nodes(v),
        None => return,
    }
}

struct ListNodeRecursive<T> {
    item: T,
    next: RefCell<Option<Box<ListNodeRecursive<T>>>>,
}

type ListHeadRecursive<T> = ListNodeRecursive<T>;

fn recur_list_empty<T>(head: &ListHeadRecursive<T>) -> bool {
    match *head.next.borrow() {
        None => return true,
        _ => return false,
    }
}

fn recur_list_add<T>(head: &mut ListHeadRecursive<T>, new_value: T) {
    let new_node = ListNodeRecursive {
        item: new_value,
        next: RefCell::new(head.next.take()),
    };
    head.next = RefCell::new(Some(Box::new(new_node)));
}


fn main() {
    let mut list_head = ListHead::<i32>::new();
    list_head.list_add(2);
    list_head.list_add(3);
    list_head.list_add(4);
    list_head.list_add(5);
    list_head.print_all();

    print_head(&list_head);

    let mut recur_list_head = ListHeadRecursive::<i32> {
        item: 0,
        next: RefCell::new(None),
    };

    if recur_list_empty(&recur_list_head) {
        println!("@recur_list_head is empty");
    } else {
        println!("@recur_list_head is not empty");
    }

    recur_list_add(&mut recur_list_head, 2);

    if recur_list_empty(&recur_list_head) {
        println!("@recur_list_head is empty");
    } else {
        println!("@recur_list_head is not empty");
    }
}
