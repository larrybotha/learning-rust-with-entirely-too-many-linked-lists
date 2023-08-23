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

    pub fn pop_front(&mut self) -> Option<T> {
        // take the old head, setting it to None
        self.head
            .take()
            .map(|old_head| {
                // take .next on the old_head's node
                match old_head.borrow_mut().next.take() {
                    // if there is a node, then...
                    Some(next_node) => {
                        // point self.head to the next node of the old node
                        self.head = Some(Rc::clone(&next_node));

                        // take .prev on the next node, removing the reference
                        next_node.borrow_mut().prev.take()
                    }
                    // else, the list is empty, and we need to drop the reference
                    // that self.tail has
                    None => self.tail.take(),
                };

                Rc::try_unwrap(old_head)
                    // convert from Result<T, E> to Option<T>
                    .ok()
                    // convert from RefCell<T> to T
                    .map(|cell| cell.into_inner())
                    .map(|node| node.elem)
            })
            .unwrap_or(None)
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

    #[test]
    fn is_push_and_poppable_at_front() {
        let mut list = List::new();
        let mut xs = [0, 1, 2];

        xs.into_iter().for_each(|x| list.push_front(x));

        xs.reverse();

        for &x in xs.iter() {
            let value = list.pop_front();

            assert_eq!(value, Some(x));
        }

        assert_eq!(list.pop_front(), None);
    }
}
