use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Node {
    value: i32,
}

pub fn test_vec() {
    let mut queue: Vec<Rc<RefCell<Node>>> = vec![];
    for i in 0..16 {
        queue.insert(0, Rc::new(RefCell::new(Node { value: i })));
    }
    for i in 0..16 {
        assert_eq!(queue.pop().unwrap().borrow().value, i);
    }
}

pub fn test_vecdeque() {
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    for i in 0..16 {
        queue.push_back(Rc::new(RefCell::new(Node { value: i })));
    }
    for i in 0..16 {
        assert_eq!(queue.pop_front().unwrap().borrow().value, i);
    }
}

pub fn test() {
    test_vec();
    test_vecdeque();
}
