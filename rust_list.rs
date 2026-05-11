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


fn main() {
    let mut list_head = ListHead::<i32>::new();
    list_head.list_add(2);
    list_head.list_add(3);
    list_head.list_add(4);
    list_head.list_add(5);
    list_head.print_all();
}
