use std::mem;

// invalid... a size cannot be determined for a recursive type
//pub enum List {
//    Empty,
//    Elem(i32, List),
//}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

// If we didn't implement Drop, the following is what the compiler
// would attempt to do when dropping a List:
//
// // #1
//impl Drop for List {
//    fn drop(&mut self) {
//          // Not allowed in real Rust - this is magic
//          // 'what the compiler would attempt' land
//          self.head.drop() // tail recursive
//    }
//}
//
//  // #2
//impl Drop for Link {
//    fn drop(&mut self) {
//        match *self {
//            Link::Empty => {},
//            Link::More(ref mut boxed_node) => {
//                  boxed_node.drop() // tail recursive, all good
//            },
//        }
//    }
//}
//
//  // #3
//impl Drop for Box<Node> {
//    fn drop(&mut self) {
//        // We're unable to drop the contents of the box before
//        // dropping the box... so we can't drop in a tail-recursive
//        // manner. This means we'll need to manually implement Drop
//        // ourselves, because we can't rely on the stack unwinding
//        // itself
//        // See https://github.com/rust-unofficial/too-many-lists/issues/239
//        self.ptr.drop(); // not tail recursive!!!
//        deallocate(self.ptr);
//    }
//}
//
//  // #4
//impl Drop for Node {
//    fn drop(&mut self) {
//        self.next.drop(); // tail recursive, all good
//    }
//}
//
// Uncommenting this code shows that that implementation doesn't work,
// because in Box we can't deallocate itself after the node is
// deallocated... this is where the tail recursion breaks

// In order to address the lack of tail recursion when dropping, we need
// to manually implement Drop for List by:
//  - looping through each node
//  - replacing each link to the next node with Empty
impl Drop for List {
    fn drop(&mut self) {
        // replace self.head with empty, assigning the value of self.head
        // to cur_link
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // While cur_link is Link::More...
        while let Link::More(mut boxed_node) = cur_link {
            // because we have Link::More, we have what the link contains,
            // i.e. boxed_node: Box<Node>
            //
            // We:
            //  - assign cur_link to boxed_node.next
            //  - replace boxed_node.next with Link::Empty
            //
            //  until our while loop exits when we get Link::Empty, i.e. we
            //  exhaust our list
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let node = Node {
            elem: value,
            // This is invalid - we're attempting to move the ownership of
            // self. head to node.next
            // When the borrow is complete, self would be partially initialised
            //next: self.head,
            // Instead we replace self.head with Link::Empty...
            next: mem::replace(&mut self.head, Link::Empty),
        };

        // ...and then set head to the new node
        self.head = Link::More(Box::new(node));

        // ... why don't we set node.next to Link::Empty from the start...?
        // because in a stack we need to point to the previous existing item!
        // We point .next to the current head, and then make the new node
        // the head
    }

    pub fn pop(&mut self) -> Option<i32> {
        // We need a reference to self.head because `match` will by default
        // move the value into its context
        // We don't own self here - we have a reference, as per the function
        // signature, so we can't move the value into the context - we have
        // to access it by reference
        //match &self.head {

        // We want to remove the existing value of head, and replace it with
        // next.node's value
        //
        // Because we want to remove something, we would ideally be working with
        // an owned value, but inside this method we only have a mutable reference
        // to self
        //
        // We can't assign self.head to node.next in the above `match`, because:
        //  - &self.head is a shared reference, which cannot be mutated
        //  - we have a mutable reference to self, which apparently means
        //      that things cannot be moved, only replaced
        //
        // Thus, we resort to mem::replace again. We:
        //  - temporarily replace self.head with Link::Empty
        //  - then set self.head to node.next
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;

                Some(node.elem)
            }
        }

        // a diverging function... i.e. doesn't return, and thus a
        // return type is inferred... in this case, None
        //unimplemented!()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

// only compile the `test` module when running tests
#[cfg(test)]
// hide our tests in a non-public `test` module
mod test {
    // Give use access to List which is in the parent scope of this module
    // Without specifying #[cfg(test)], Rust warns that List is
    // unused
    use super::List;

    // run this function when `cargo test` is run
    #[test]
    fn basics() {
        let mut list = List::new();

        // popping empty list returns None
        assert_eq!(list.pop(), None);

        [1, 2, 3].map(|x| list.push(x));

        // pops work
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push some more
        [4, 5].map(|x| list.push(x));

        // pop until we stop
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
