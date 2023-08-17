use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
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

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
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

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();

            &node.elem
        })
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
