use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub val: u32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: u32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn binary_tree_create(vals: Vec<u32>) -> Option<Rc<RefCell<Node>>> {
    let mut tree = None;
    for val in vals {
        tree = binary_tree_insert(tree, val)
    }
    tree
}

pub fn binary_tree_insert(
    mut node: Option<Rc<RefCell<Node>>>,
    val: u32,
) -> Option<Rc<RefCell<Node>>> {
    if node.is_some() {
        match node.take() {
            Some(node_taken) => {
                let v = node_taken.borrow().val;
                if val < v {
                    if node_taken.borrow().left.is_none() {
                        node_taken.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(val))));
                    } else {
                        let _ = binary_tree_insert(node_taken.borrow().left.clone(), val);
                    }
                } else {
                    if node_taken.borrow().right.is_none() {
                        node_taken.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(val))));
                    } else {
                        let _ = binary_tree_insert(node_taken.borrow().right.clone(), val);
                    }
                }
                node = Some(node_taken);
                node
            }
            None => {
                panic!("impossible")
            }
        }
    } else {
        Some(Rc::new(RefCell::new(Node::new(val))))
    }
}

pub fn binary_tree_inorder_traverse(mut node: Option<Rc<RefCell<Node>>>) -> Vec<u32> {
    let mut v = vec![];
    if node.is_some() {
        match node.take() {
            Some(node_taken) => {
                v.push(node_taken.borrow().val);
                let v_left = binary_tree_inorder_traverse(node_taken.borrow().left.clone());
                v.extend(v_left);
                let v_right = binary_tree_inorder_traverse(node_taken.borrow().right.clone());
                v.extend(v_right);
                node = Some(node_taken);
            }
            None => {
                panic!("impossible")
            }
        }
    }
    v
}

pub fn test() {
    let vals = vec![5, 4, 2, 1, 0, 3, 8, 9, 6, 7];
    let tree = binary_tree_create(vals);
    let traverse = binary_tree_inorder_traverse(tree);
    assert_eq!(vec![5, 4, 2, 1, 0, 3, 8, 6, 7, 9], traverse);
}
