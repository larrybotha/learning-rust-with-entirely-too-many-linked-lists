use std::mem;

// push attempt 1
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // .push attempt 1: using mem::replace
    //
    // This implementation fails because we attempt to move new_tail_node
    // to two places
    pub fn push(&mut self, elem: T) {
        let new_tail_node = Box::new(Node { elem, next: None });

        // 1. replace the current tail with the new node
        let old_tail = mem::replace(&mut self.tail, Some(new_tail_node));

        // 2. we get an Option for the old tail
        //      - if Some, point that node to the new node
        //      - else, point the head to the new node
        match old_tail {
            Some(old_node) => {
                // This fails, because:
                //      - Box does not implement Copy - therefore where old_tail
                //          was instantiated, new_tail_node has already been moved
                old_node.next = Some(new_tail_node);
            }
            None => {
                self.head = Some(new_tail_node);
            }
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}
