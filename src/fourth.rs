use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Node::new(elem);

        // using the "if let" pattern
        //if let Some(old_head) = self.head.take() {
        //    self.head = Some(Rc::clone(&node));
        //    node.borrow_mut().next = Some(Rc::clone(&old_head));
        //    old_head.borrow_mut().prev = Some(Rc::clone(&node));
        //} else {
        //    self.head = Some(Rc::clone(&node));
        //    self.tail = Some(Rc::clone(&node));
        //}

        // using a more idiomatic "match" pattern
        match self.head.take() {
            // if we have a head, set the appropriate references on the new
            // head node and the old head node
            Some(old_head) => {
                // set .next on the new node to the old head's node
                node.borrow_mut().next = Some(Rc::clone(&old_head));
                // set .prev on the old head's node to the new node
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                // set the new node as head
                self.head = Some(Rc::clone(&node));
            }
            // otherwise, point the head and tail to the new node
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn new_has_no_links() {
        let list: List<i32> = List::new();

        println!("head: {:?}", list.head);
        println!("tail: {:?}", list.tail);
    }
}
