use core::ptr::NonNull;

struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node { item, next: None }
    }
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }
}

//------------------------------------------------------
//  Test Functions
//------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let test_node_int = Node::new(10);
        let test_node_float = Node::new(10.);
        let test_node_str = Node::new("Hello, World!");
        let test_node_string = Node::new(String::from("Hello, World!"));
    }

    #[test]
    fn create_linked_list() {
        let test_linked_list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let test_linked_list: SinglyLinkedList<f32> = SinglyLinkedList::new();
        let test_linked_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        let test_linked_list: SinglyLinkedList<String> = SinglyLinkedList::new();
    }
}
