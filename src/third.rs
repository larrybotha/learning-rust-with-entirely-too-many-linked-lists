use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

// We need a struct that will implement the Iterator trait
// Why do we define a lifteime in the struct declaration?
//  Because one or more of the fields contains a reference
// Why do we have a reference?
//  Because our value is derived from the contents of an Rc, which is a
//  reference
//  This also means we can't implement IterMut for our List with our
//  current definitions - Rc on its own doesn't allow for mutable references
//  because we have _multiple references_ - Rust's borrowing rules
//  Also, by convention, .iter returns an iterator that yields
//  references, as opposed to .into_iter which yields and consumes values
pub struct Iter<'a, T> {
    // it has a field
    next: Option<&'a Node<T>>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // create an associated function which allows us to return Iter -
    // the struct that will be implementing Iterator
    pub fn iter(&self) -> Iter<'_, T> {
        // return our Iter struct
        Iter {
            // with its initial value as the head of the list
            // We deference the value using .as_deref, which does the
            // following:
            //  - coerces Option<Rc<Node<T>>> to Option<Node<T>>
            //  - converts Option<Node<T>> to Option<&Node<T>>, which is
            //      what we need as defined by Iter's type
            next: self.head.as_deref(),
        }
    }

    pub fn prepend(&mut self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(self) -> Self {
        List {
            // naive: unwrapping after mapping
            //head: self
            //    .head
            //    // get a nested option
            //    .map(|node| node.next.clone())
            //    // unwrap the value
            //    .unwrap_or(None),

            // or, use .and_then, which is similar to flatmap for removing a level
            // of nesting
            head: self
                .head
                .as_ref()
                // node.next.clone() returns an Option, .and_then removes it
                .and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head
            // get a reference to the node at head
            .as_ref()
            // return a reference to the value inside the node
            .map(|node| &node.elem)
    }
}

// we implement Iterator for Iter.
// Iter has a lifetime parameter, so when we implement a trait, we need to
// specify that the implementation has a lifetime parameter
impl<'a, T> Iterator for Iter<'a, T> {
    // Iterator has an associated type called Item. Item is the value
    // that is returned by .next()
    type Item = &'a T;

    // The minimum implementation requirement for Iterator is .next()
    fn next(&mut self) -> Option<Self::Item> {
        // self.next, the attribute on Iter (i.e. the struct can have both fields
        // and associated functions with the same name) has the type
        // Option<&'a Node<T>>
        // We need to:
        //  - return a Option<&'a T>
        //  - set Iter.next to the next node
        self.next.map(|node| {
            // as when we initialised Iter, each node's .next value will have the
            // type Option<Rc<Node<T>>>
            // We do the same as during initialisation; use Option::as_deref to:
            //  - coerce Option<Rc<Node<T>>> to Option<Node<T>>
            //  - convert Option<Node<T>> to Option<&Node<T>>, which is the type
            //      that Iter::next expects
            self.next = node.next.as_deref();

            // we return a reference to the node's element
            &node.elem
        })
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // .take the head so that it is consumed and dropped
        let mut current_node = self.head.take();

        // while the current node is not None...
        while let Some(node_ref) = current_node {
            // attempt to unwrap the Rc
            //  - if we get Ok, then:
            //      - we know there's only 1 reference to the internal value
            //      - we can take ownership of that value
            //      - we decrement the reference count to 0
            //      - Rust then drops node_ref. We can see this by attempting to
            //          view the Rc::strong_count inside the block:
            //          - Rust indicates that node_ref has been moved to
            //              Rc::try_unwrap
            //          - Rust complains inside the block that we're attempting to
            //              borrow a moved value
            //              i.e. the value has been moved into Rc::try_unwrap, and
            //              then inside the method has been dropped
            //  - if we don't, then :
            //      - there's some other list pointing to the node
            //      - we need to stop dropping values
            if let Ok(mut node) = Rc::try_unwrap(node_ref) {
                // we know we're working with a node that has only a single
                // reference to it - it's safe to drop this node, so we:
                //  - .take the node's next value
                //  - let the node get dropped at the end of this scope
                current_node = node.next.take();
            } else {
                // stop processing the list if there are other references to the
                // current node!
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();

        let list = list.prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
