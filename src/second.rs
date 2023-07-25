struct Node<T> {
    elem: T,
    next: Link<T>,
}

// This is actually a poor reimplementation of Option
//enum Link {
//    Empty,
//    More(Box<Node>),
//}

// replace it with a type alias
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        //let mut cur_link = mem::replace(&mut self.head, None);
        //
        // mem::replace(&mut option, None) is such a common idiom that...
        // Rust provides sugar for it with .take()
        //
        // Boom!
        //
        // Option::take ≡ mem::replace(&mut some_option, None)
        //
        // Option::take does the following:
        //  1. pulls the value out of the option
        //  2. replaces the contents of the option with None
        //  3. returns the value for  us to do w/e with - assign to a variable,
        //      map over it, etc.
        //
        // i.e. when one sees
        //          mem::replace(&mut option, None)
        // one should replace it with
        //          option.take()
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }


    pub fn push(&mut self, value: T) {
        let node = Node {
            elem: value,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        //match self.head.take() {
        //    None => None,
        //    Some(node) => {
        //        self.head = node.next;

        //        Some(node.elem)
        //    }
        //}
        //
        // match option { None => {}, Some(x) => {}}
        // is also such a common idiom that it 'became' Option::map, instead
        //
        // i.e. when one sees:
        //      match option.take() { None => None, Some(x) => Some(y) }
        // one should refactor to use
        //      option.map(|x| y)
        self.head.take().map(|node| {
            self.head = node.next;

            node.elem
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
