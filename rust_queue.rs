use std::cell::RefCell;
use std::rc::{Rc, Weak};


type NodeConnectorType<_Tp> = Rc<RefCell<Box<QueueNode<_Tp>>>>;

struct QueueNode<_Tp> {
    item: _Tp,
    next: Option<NodeConnectorType<_Tp>>,
}
impl<_Tp> QueueNode<_Tp> {
    fn new(value: _Tp) -> Self {
        Self {
            item: value,
            next: None,
        }
    }
    fn ref_inner<'a>(&'a self) -> &'a _Tp {
        &self.item
    }
}

impl<_Tp> Drop for QueueNode<_Tp> {
    fn drop(&mut self) {
        println!("Node dtor is called.");
        self.next = None;
    }
}

type QueueHeadType<_Tp> = NodeConnectorType<_Tp>;
type QueueTailType<_Tp> = Weak<RefCell<Box<QueueNode<_Tp>>>>;

struct Queue<_Tp> {
    head: Option<QueueHeadType<_Tp>>,
    tail: Option<QueueTailType<_Tp>>,
}

impl<_Tp> Queue<_Tp> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn construct_connector(node: QueueNode<_Tp>) -> NodeConnectorType<_Tp> {
        Rc::new(RefCell::new(Box::new(node)))
    }

    fn enqueue(&mut self, value: _Tp) {
        let new_node = QueueNode::<_Tp>::new(value);
        let node_connector = Self::construct_connector(new_node);
        if let None = self.head {
            self.tail = Some(Rc::downgrade(&node_connector));
            self.head = Some(node_connector);
        } else {
            let tail_rc = self.tail
                .take()
                .expect("Not None")
                .upgrade()
                .expect("Should refer to an entity.");
            self.tail = Some(Rc::downgrade(&node_connector));
            tail_rc.borrow_mut().next = Some(node_connector);
        }
    }

    fn dequeue(&mut self) -> Option<NodeConnectorType<_Tp>> {
        if let None = self.head {
            return None;
        }
        // move head node out from queue.
        let ret_rc = self.head.take().unwrap();
        if let Some(v) = ret_rc.borrow_mut().next.take() {
            self.head = Some(v);
        } else {
            self.tail = None;
        }
        Some(ret_rc)
    }
}

fn main() {
    let mut queue = Queue::<i32>::new();
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(5);
    queue.enqueue(6);
    queue.enqueue(7);
    let node_connector = queue.dequeue();
    match node_connector {
        None => println!("empty queue"),
        Some(v) => println!("item : {}",
                            v.borrow().ref_inner()),
    }
    let node_connector = queue.dequeue();
    match node_connector {
        None => println!("empty queue"),
        Some(v) => println!("item : {}",
                            v.borrow().ref_inner()),
    }
    let node_connector = queue.dequeue();
    match node_connector {
        None => println!("empty queue"),
        Some(v) => println!("item : {}",
                            v.borrow().ref_inner()),
    }
}
