use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
}

pub fn test() {
    let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
    for i in 0..16 {
        stack.push(Rc::new(RefCell::new(Node { value: i })));
    }
    for i in (0..16).rev() {
        assert_eq!(stack.pop().unwrap().borrow().value, i);
    }
}
