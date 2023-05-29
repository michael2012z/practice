use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    value: i32,
}

pub fn test() {
    let mut map = HashMap::new();
    map.insert("abc".to_string(), Rc::new(RefCell::new(Node { value: 1 })));
    map.insert("hij".to_string(), Rc::new(RefCell::new(Node { value: 2 })));
    map.insert("opq".to_string(), Rc::new(RefCell::new(Node { value: 3 })));
    map.insert("xyz".to_string(), Rc::new(RefCell::new(Node { value: 4 })));
    assert_eq!(map.contains_key("abc"), true);
    assert_eq!(map.contains_key("def"), false);
    map.remove("abc");
    assert_eq!(map.contains_key("abc"), false);
    assert_eq!(map.get("xyz").unwrap().borrow().value, 4);
    map.get("xyz").unwrap().borrow_mut().value = 5;
    assert_eq!(map.get("xyz").unwrap().borrow().value, 5);
}
