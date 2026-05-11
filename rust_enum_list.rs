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

enum EnumListNode<T> {
    item(Option<T>, Option<Box<EnumListNode<T>>>),
    nil,
}

fn enum_list_empty<T>(head: &EnumListNode<T>) -> bool {
    match head {
        EnumListNode::<T>::nil => return true,
        _ => return false,
    }
}

fn enum_list_add<T>(head: &mut EnumListNode<T>, new_value: T) {
    match head {
        EnumListNode::<T>::item(v, n) => {
            let new_node = EnumListNode::<T>::item(
                Some(new_value),
                n.take(),
            );
            *head = EnumListNode::<T>::item(v.take(), Some(Box::new(new_node)));
        },
        _ => *head = EnumListNode::<T>::item(Some(new_value), None),
    }
}


fn main() {
    let mut list_head: ListNode<_> = ListNode::new();
    list_head = list_head.list_add(2);
    list_head = list_head.list_add(3);
    list_head = list_head.list_add(4);
    list_head.print_all();

    let mut enum_list_head = EnumListNode::nil;
    if enum_list_empty(&enum_list_head) {
        println!("@enum_list_empty is empty");
    } else {
        println!("@enum_list_empty is not empty");
    }

    enum_list_add(&mut enum_list_head, 2);
    if enum_list_empty(&enum_list_head) {
        println!("@enum_list_empty is empty");
    } else {
        println!("@enum_list_empty is not empty");
    }

    enum_list_add(&mut enum_list_head, 4);
    if enum_list_empty(&enum_list_head) {
        println!("@enum_list_empty is empty");
    } else {
        println!("@enum_list_empty is not empty");
    }
}
