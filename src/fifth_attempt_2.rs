/// List attempt 2 - using a reference without ownership for working with
/// the tail of the list
///
/// Contrast this implementation with ./fifth_attempt_1.rs

pub struct List<'a, T> {
    head: Link<T>,
    // instead of a Link, which underneath is a Box, let's rather
    // use a reference to the value inside the Box, which is Node<T>
    //
    // This reference needs to be mutable, because when a new value is pushed
    // onto the the queue, we need to assign .next of the current tail to the
    // new value
    //
    // Because we have a reference inside our Struct, we need a lifetime parameter
    // in our struct definition to indicate to the compiler that our struct needs
    // to live for _at least_ as long as the referenced value
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// because List now has a lifetime
impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // .push attempt 2: using self.tail.take() so we can defer assignment
    // of the new tail
    //
    // The compiler indicates that we need to specify a lifetime parameter for
    // self - we tell the compiler that the lifetime of the instance is that of
    // the lifetime of the reference it contains...
    //
    // lifetime of self == lifetime self.tail
    //
    // So... the lifetime of the instance must be at least as long as the value at
    // its tail, but its tail must also live at least as long as the instance...?!
    //
    // Rust allows this to compile... but why...?
    //
    // Because this valid, and only locks up once we have a _mutable_ reference
    // assigned to the tail
    //
    // The problem arises when we _use_ push - by specifying that self has a
    // lifetime of 'a, when we call push, we tell the compiler that a mutable
    // reference to self exists, and we can't borrow self again until 'a is over
    //
    // This can't happen, because we've marked the lifetime of the instance as the
    // lifetime of its contained value - the instance's lifetime is cyclacle, so
    // the reference will never be removed!
    //
    // We can call .push once, before there is a mutable reference to self. After
    // this, we can no longer call push or pop because both methods require mutable
    // access to self - we've locked our struct!
    pub fn push(&'a mut self, elem: T) {
        let new_tail_node = Box::new(Node { elem, next: None });

        let new_tail = match self.tail.take() {
            // there is an existing tail, set the old tail to point to the new tail
            Some(old_tail) => {
                old_tail.next = Some(new_tail_node);

                // Return a mutable reference to the new node
                old_tail.next.as_deref_mut()
            }
            // there is no tail, therefore set the head to point to the new tail
            None => {
                self.head = Some(new_tail_node);

                // return a mutable reference to the new node
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.next;

                Some(old_head.elem)
            }
            None => {
                self.tail = None;

                None
            }
        }
    }
}

impl<'a, T> Default for List<'a, T> {
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

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
