use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    val: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: u32, next: Option<Rc<RefCell<Node>>>) -> Self {
        Node { val, next }
    }
}

// Create a single linked list from a vector of u32
fn singly_linked_list_create(vals: Vec<u32>) -> Option<Rc<RefCell<Node>>> {
    // Create a head of the list.
    // Using an extra node increased the overhead of the algorithm, but can save you a lot of trouble.
    let list = Some(Rc::new(RefCell::new(Node::new(0, None))));

    // Insert each value to the list
    for val in vals {
        singly_linked_list_insert(list.clone(), val);
    }

    // Skp the head. Return the real elements of the list.
    list.unwrap().borrow().next.clone()
}

// Create a Node to hold the value. Insert the node to the list in the order of ascending.
// After the insertion, the list is sorted.
fn singly_linked_list_insert(list: Option<Rc<RefCell<Node>>>, val: u32) {
    // Use a current and next "pointer" to move along the list
    let mut current = list.clone();
    let mut next = current.clone().map(|x| x.borrow().next.clone()).flatten();

    // Move to a location where:
    // * The "current" pointer holds a value less/equal than "val"
    // * The "next" pointer is None or holds a larger value than "val"
    // So the new node should be inserted between "current" and "next".
    while next.is_some() && next.clone().map(|x| x.borrow().val).unwrap() <= val {
        current = next.clone();
        next = current.clone().map(|x| x.borrow().next.clone()).flatten();
    }

    // Set the new node to the "next" field of current ptr, and move forward the current ptr
    match current.take() {
        Some(current_taken) => {
            current_taken.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(val, next))));
            // Critical!
            // After "current.take()", the content of "current" is taken away, now "current" is None.
            // Do not forget to set the taken content back after modifying it.
            current = current_taken.borrow().next.clone();
        }
        None => {
            panic!("impossible")
        }
    };
}

// Traverse the linked list and build a vector of values.
// The vector should be sorted.
fn singly_linked_list_traverse(list: Option<Rc<RefCell<Node>>>) -> Vec<u32> {
    let mut traverse = vec![];
    let mut current = list.clone();

    while current.is_some() {
        match current.take() {
            Some(current_taken) => {
                traverse.push(current_taken.borrow().val);
                current = current_taken.borrow().next.clone();
            }
            None => {}
        }
    }

    traverse
}

pub fn test() {
    let vals = vec![5, 4, 2, 1, 0, 3, 8, 9, 6, 7];
    let list = singly_linked_list_create(vals.clone());
    let traverse = singly_linked_list_traverse(list);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], traverse);
}
