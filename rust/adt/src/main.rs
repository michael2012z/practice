mod binary_tree;
mod hashmap;
mod playground;
mod queue;
mod set;
mod singly_linked_list;
mod stack;
mod vector_of_vectors;

fn main() {
    playground::test();
    singly_linked_list::test();
    binary_tree::test();
    vector_of_vectors::test();
    stack::test();
    queue::test();
    hashmap::test();
    set::test();
}
