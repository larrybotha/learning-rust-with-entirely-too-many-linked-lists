use std::mem;

// ***** Attempt 1 *****
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

//pub struct List<'a, T> {
//    head: Link<T>,
//    tail: Option<&'a mut Node<T>>,
//}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // ***** Attempt 1 *****
    pub fn push(&mut self, elem: T) {
        let new_tail_node = Box::new(Node { elem, next: None });

        // 1. replace the current tail with the new node
        // 2. we get an Option for the old tail
        //      - if Some, point that node to the new node
        //      - else, point the head to the new node
        let old_tail = mem::replace(&mut self.tail, Some(new_tail_node));

        match old_tail {
            Some(old_node) => {
                old_node.next = Some(new_tail_node);
            }
            None => {
                self.head = Some(new_tail_node);
            }
        }
    }

    // ***** Attempt 2 *****
    //pub fn push(&mut self, elem: T) {
    //    let new_tail_node = Box::new(Node { elem, next: None });

    //    // Set tail to None, so that we can get the new tail
    //    // The new tail should be:
    //    //  - the old tail's
    //    //let new_tail = match self.tail.take() {
    //    //    Some(old_tail_node) => {
    //    //        old_tail_node.next = Some(new_tail_node);
    //    //        old_tail_node.next.as_deref_mut()
    //    //    }
    //    //    None => {
    //    //        self.head = Some(new_tail_node);
    //    //        self.head.as_deref_mut()
    //    //    }
    //    //};

    //    //self.tail = new_tail;
    //}

    pub fn pop(&'a mut self) -> Option<T> {
        match self.head.take() {
            None => {
                self.tail = None;

                None
            }
            Some(old_head) => {
                let node = *old_head;
                self.head = node.next;

                Some(node.elem)
            }
        }
    }
}

impl<T> Default for List<'_, T> {
    fn default() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    //#[test]
    //fn basics() {
    //    let mut list = List::new();
    //    let xs = vec![1, 2, 3];

    //    assert!(list.head.is_none());

    //    xs.iter().for_each(|&x| list.push(x));

    //    for &x in xs.iter().rev() {
    //        let value = list.pop();

    //        assert_eq!(value, Some(x));
    //    }

    //    assert!(list.head.is_none());
    //}
}
