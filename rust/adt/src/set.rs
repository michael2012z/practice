use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    value: i32,
}

pub fn test() {
    let mut set = HashSet::new();
    set.insert(Node { value: 1 });
    set.insert(Node { value: 2 });
    set.insert(Node { value: 3 });
    assert_eq!(set.contains(&Node { value: 1 }), true);

    let mut set_ = HashSet::new();
    set_.insert(Node { value: 4 });
    set_.insert(Node { value: 2 });
    set_.insert(Node { value: 3 });

    assert_eq!(set.intersection(&set_).count(), 2);
}
