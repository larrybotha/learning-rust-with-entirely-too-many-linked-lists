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
    // to two places, annotated at φ and θ
    //
    // In addition to attempting to move the non-Copy new_tail_node value to
    // two locations, we have an additional problem _if_ that were possible:
    //
    //      Box owns the value it contains - when we dropped Box, it would
    //      double-free the tail:
    //          - once when self.tail is freed
    //          - again where the nth-1 node contains the same boxed value
    pub fn push(&mut self, elem: T) {
        let new_tail_node = Box::new(Node { elem, next: None });

        // 1. replace the current tail with the new node
        //
        // φ: Something else is happening here, too - Box is not Copy, so we're
        // moving new_tail_node into the Option
        //
        // We're also pulling the old boxed value out of self.tail as a variable,
        // i.e. old_tail. old_tail is only used in the match statement, after
        // which we do nothing with it... it will get dropped at the end of
        // every push!
        let old_tail = mem::replace(&mut self.tail, Some(new_tail_node));

        // 2. we get an Option for the old tail
        //      - if Some, point that node to the new node
        //      - else, point the head to the new node
        match old_tail {
            Some(old_node) => {
                // θ: as at φ, we're attempting to move new_tail_node into the Option
                // _again_ - this is possible with values that implement Copy, but
                // Box doesn't! This will result in a move, but because we've
                // already moved the value at φ, we're going to get a compiler
                // error
                old_node.next = Some(new_tail_node);
            }
            None => {
                // we have the same problem here as with θ - if the match statement
                // branches to None, we'll be moving the already moved new_tail_node
                // into this Option
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
