use std::cell::RefCell;

/* type alias */
type NodeConnectorType<_Tp> = RefCell<Box<TreeNode<_Tp>>>;

fn construct_NodeConnector_from_node<_Tp>(node: TreeNode<_Tp>)
-> NodeConnectorType<_Tp>
where
    _Tp: std::cmp::PartialOrd
{                         
    RefCell::new(Box::new(node))
}

/* TreeNode<> - represents a tree node */
struct TreeNode<_Tp: std::cmp::PartialOrd> {
    item: _Tp,
    left: Option<NodeConnectorType<_Tp>>,
    right: Option<NodeConnectorType<_Tp>>,
}

impl<_Tp: std::cmp::PartialOrd + std::fmt::Display> TreeNode<_Tp> {
    fn new(value: _Tp) -> Self {
        Self {
            item: value,
            left: None,
            right: None,
        }
    }

    fn become_left(root_ref: &NodeConnectorType<_Tp>, new_node: NodeConnectorType<_Tp>) {
        root_ref.borrow_mut().left = Some(new_node);
    }

    fn become_right(root_ref: &NodeConnectorType<_Tp>, new_node: NodeConnectorType<_Tp>) {
        root_ref.borrow_mut().right = Some(new_node);
    }

    fn take_left(node: &NodeConnectorType<_Tp>) -> Option<NodeConnectorType<_Tp>> {
        match node.borrow_mut().left.take() {
            None => None,
            Some(v) => Some(v),
        }
    }

    fn take_right(node: &NodeConnectorType<_Tp>) -> Option<NodeConnectorType<_Tp>> {
        match node.borrow_mut().right.take() {
            None => None,
            Some(v) => Some(v),
        }
    }

    fn is_left_path(root: &NodeConnectorType<_Tp>, node: &TreeNode<_Tp>) -> bool {
        if node.item < root.borrow().item {
            true
        } else {
            false
        }
    }

    fn is_right_path(root: &NodeConnectorType<_Tp>, node: &TreeNode<_Tp>) -> bool {
        !Self::is_left_path(root, node)
    }

    /*
     * tree_add - the general tree insertion routine
     * @root:     the start root,get ownership
     * @new_node: the node will be inserted,get ownership
     * return:    the @root,give back ownership
     * # we also can borrow @root instead get ownership.
     *   in that case,when we need get into next level,we must
     *   take the next node from @root at first,recursively invoke this
     *   function again,pass it the next node as reference,and @new_node.
     *   after the call chain ends,we need to re-insert the passed
     *   next node back to @root.
     *   if we want use a boolean value to tell whether the insertion is
     *   succeed,we can follow this method.
     *   but the failure of insertion would only happens when there been
     *   such node is existing.
     *   primitive FIND can does such kind of checking,we do not need to
     *   do that in this function.
     */
    fn tree_add(root: NodeConnectorType<_Tp>, new_node: NodeConnectorType<_Tp>)
                -> NodeConnectorType<_Tp>
    {
        if root.borrow().item == new_node.borrow().item {
            return root;
        }

        if Self::is_left_path(&root, &*new_node.borrow()) {
            // left
            match Self::take_left(&root) {
                None => Self::become_left(&root, new_node),
                Some(v) => root.borrow_mut().left = Some(Self::tree_add(v, new_node)),
            }
        } else {
            // right
            match Self::take_right(&root) {
                None => Self::become_right(&root, new_node),
                Some(v) => root.borrow_mut().right = Some(Self::tree_add(v, new_node)),
            }
        }

        root
    }

    /*
     * when we define FIND primitive,we mut let the primitive returns a reference to
     * that tree node instead to return ownership.
     * the tree node's ownership is held by RefCell<> object.
     * for DELETE primitive,we can let it returns the ownership of the old tree node.
     */

    fn tree_preorder_traversal(root: &NodeConnectorType<_Tp>) {
        println!("item : {}", root.borrow().item);

        if let Some(next_ref) = root.borrow().left.as_ref() {
            Self::tree_preorder_traversal(next_ref);
        }

        if let Some(next_ref) = root.borrow().right.as_ref() {
            Self::tree_preorder_traversal(next_ref);
        }
    }
}

/* root wrapper */
struct TreeRoot<_Tp: std::cmp::PartialOrd> {
    tree: Option<NodeConnectorType<_Tp>>,
}
impl<_Tp: std::fmt::Display + std::cmp::PartialOrd> TreeRoot<_Tp> {
    fn new() -> Self {
        Self {
            tree: None,
        }
    }

    fn insert(&mut self, value: _Tp) {
        let new_node = TreeNode::new(value);
        match self.tree.take() {
            None => {
                let new_node_connector = construct_NodeConnector_from_node(new_node);
                self.tree = Some(new_node_connector);
            },
            Some(root) => {
                let new_node_connector = construct_NodeConnector_from_node(new_node);
                self.tree = Some(TreeNode::tree_add(root, new_node_connector));
            },
        }
    }

    fn preorder_traversal(&self) {
        match self.tree.as_ref() {
            None => return,
            Some(root) => TreeNode::tree_preorder_traversal(root),
        }
    }
}

fn main() {
    let mut tree = TreeRoot::<i32>::new();
    for i in (0..=6).rev() {
        tree.insert(i);
    }
    for i in 7..=11 {
        tree.insert(i);
    }
    tree.preorder_traversal();
    dbg!(tree);
}
