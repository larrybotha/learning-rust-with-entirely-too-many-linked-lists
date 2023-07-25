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

    pub fn peek(&self) -> Option<&T> {
        // The following fails, because .map moves node into the closure,
        // and out of head
        //self.head.map(|node| &node.elem)

        // we have to use .as_ref because .map takes self by value, which
        // would move the value _out_ of the head, which we don't want
        // .as_ref switches from &Option<T> to Option<&T> - in the lesson
        // it's said that the value is demoted from Option<T> to an Option
        // with a reference to the value
        self.head.as_ref().map(|node| &node.elem)
    }

    // we need to declare that the return type is mutable
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // Option::as_mut returns a mutable reference to the caller
        self.head.as_mut().map(|node| &mut node.elem)
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

    #[test]
    fn peek() {
        let mut list = List::new();
        let mut xs: [i32; 5] = (0..5).collect::<Vec<i32>>().try_into().unwrap();

        xs.into_iter().for_each(|x| list.push(x));

        xs.reverse();

        for x in xs {
            assert_eq!(list.peek(), Some(&x));
            list.pop();
        }
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        let mut xs: [i32; 5] = (0..5).collect::<Vec<i32>>().try_into().unwrap();

        xs.into_iter().for_each(|x| list.push(x));

        xs.reverse();

        for x in xs {
            // We don't destructure y here using `&mut y`
            // If we did, we'd be left with i32, when what we want is to operate
            // on the mutable reference to i32, i.e. we want &mut i32 inside the
            // closure
            // Additionally, we need to return y in the closure, because .map
            // expects a return value. We can't use Option::for_each because there
            // is no such thing, although we could use Option::iter::for_each to
            // first turn it into an iterator, but that brings other compiler
            // nightmares...
            //list.peek_mut().map(|y| {
            //    *y += 1;
            //    y
            //});

            // We could also use if let, which in this case turns out to be more
            // terse, yet potentially less readable:
            if let Some(y) = list.peek_mut() {
                *y += 1
            };

            let value = list.peek();

            assert_eq!(value, Some(&(x + 1)));

            list.pop();
        }
    }

}
