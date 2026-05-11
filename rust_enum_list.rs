enum ListNode<T> {
    item(T, Box<ListNode<T>>),
    nil,
}

impl<T: std::fmt::Display> ListNode<T> {
    fn new() -> Self {
        Self::nil
    }

    fn set_item(self, v: T) -> Self {
        match self {
            Self::item(value, next) => {
                Self::item(v,  next)
            },
            Self::nil => self,
        }
    }

    fn list_add(self, new_value: T) -> Self {
        match self {
            Self::item(v, n) => {
                let new_node = Self::item(new_value, n);
                Self::item(v, Box::new(new_node))
            },
            Self::nil => Self::item(new_value, Box::new(Self::nil)),
        }
    }

    fn print_all(&self) {
        match self {
            Self::item(v, n) => {
                println!("{}", v);
                Self::print_all(&*n);
            }
            _ => return,
        }
    }
}

fn main() {
    let mut list_head: ListNode<_> = ListNode::new();
    list_head = list_head.list_add(2);
    list_head = list_head.list_add(3);
    list_head = list_head.list_add(4);
    list_head.print_all();
}
