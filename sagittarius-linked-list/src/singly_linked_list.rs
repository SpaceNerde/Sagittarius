use core::cell::Cell;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    marker: PhantomData<Cell<T>>,
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
    pub const fn new() -> Self {
        SinglyLinkedList {
            head: None,
            marker: PhantomData,
        }
    }

    unsafe fn push_front_node(&mut self, item: NonNull<Node<T>>) {
        unsafe {
            (*item.as_ptr()).next = self.head;
            let item = Some(item);
            self.head = item;
        }
    }

    fn push_front(&mut self, item: T) {
        let node = Box::new(Node::new(item));
        let nonnull_node = NonNull::new(Box::into_raw(node)).unwrap();

        unsafe {
            self.push_front_node(nonnull_node);
        }
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
        let _test_node_int = Node::new(10);
        let _test_node_float = Node::new(10.);
        let _test_node_str = Node::new("Hello, World!");
        let _test_node_string = Node::new(String::from("Hello, World!"));
    }

    #[test]
    fn create_linked_list() {
        let _test_linked_list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let _test_linked_list: SinglyLinkedList<f32> = SinglyLinkedList::new();
        let _test_linked_list: SinglyLinkedList<&str> = SinglyLinkedList::new();
        let _test_linked_list: SinglyLinkedList<String> = SinglyLinkedList::new();
    }

    #[test]
    fn push_node() {
        let mut test_linked_list: SinglyLinkedList<i8> = SinglyLinkedList::new();
        test_linked_list.push_front(10);
    }
}
