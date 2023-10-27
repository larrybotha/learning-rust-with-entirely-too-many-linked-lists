// push attempt 2
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>, //tail: Option<&'a mut Node<T>>,
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

    // .push attempt 2: using self.tail.take() so we can defer assignment
    // of the new tail
    pub fn push(&mut self, elem: T) {
        let new_tail_node = Box::new(Node { elem, next: None });

        // 1. take the current tail, so that we can set it later
        // 2. we get an Option:
        //      - if Some:
        //          - set the old tail's next value to the new node
        //          - return the dereferenced new tail
        //      - else:
        //          - set the head to the new node
        //          - return the dereferenced new tail
        let new_tail = match self.tail.take() {
            // if we have elements in the list...
            Some(mut old_tail) => {
                // assign the old tail's next value to the new node
                old_tail.next = Some(new_tail_node);

                // Pull the node out of the box, and return it as mutable
                //
                // i.e. Some(Box<Node>) => Some(&mut Node)
                let x = old_tail.next.as_deref_mut();

                x
            }
            // if we're adding the first element in the list...
            None => {
                self.head = Some(new_tail_node);

                // convert from Option<Box<Node>> to Option<&mut Node>
                let x = self.head.as_deref_mut();

                x
            }
        };

        self.tail = new_tail;
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
