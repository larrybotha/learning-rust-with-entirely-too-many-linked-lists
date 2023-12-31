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

// #Iterator 1 - we create a tuple struct which wraps our List
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // #Iterator 2 - we add an associated function which returns
    // IntoIter wrapping our List
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Instead of entirely eliding the lifetime, we can indicate that there
    // _is_ a lifetime, but because of lifetime elision rules it can be inferred
    pub fn iter(&self) -> Iter<'_, T> {
        //pub fn iter(&self) -> Iter<T> {
        //
        // We can elide the lifetimes in this method because of the following
        // lifetime elision because...
        //  - the lifetime of a returned value of a function that only takes
        //      one argument is the lifetime of that argument
        //pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: {
                // .as_deref() does the following:
                //  - changes Option<T> to Option<&T>, i.e. creating a reference to the value in
                //  the Option
                //  - converts Option<Bos<Node<T>>> into Option<&Box<Node<T>>>, and then
                //  - dereferences the Box, resulting in Option<&Node<T>>
                self.head.as_deref()

                // We could also have done the following:
                //
                //self.head                           // here we have Box<Node<T>>
                //    .as_ref()                       // now we have &Box<Node<T>>
                //    .map(|node| node.as_ref())      // return &Node<T>
                //
                // or a nasty-looking reference and double-dereference
                //self.head
                //    .as_ref()
                //    .map(|node| &**node)
                //
                // or even with turbofish
                //self.head
                //    .as_ref()
                //    .map::<&Node<T>, _>(|node| node)
            },
        }
    }

    // Define a public method that returns a mutable iterable of the list
    // We return IterMut where IterMut::next is a private attribute that holds
    // a mutable reference to the node at the head of the list
    // This .next is different from the public associated function .next we
    // define when implementing Iterator - that function uses _this_ .next
    // attribute in order to get the value out of the node, and return a mutable
    // reference to the value inside the node
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            // We use Option::as_deref_mut because:
            //  - we want deref coercion to get Node<T> out of Box<Node<T>>
            //  - we want to return a reference to the Node inside the Box
            //  - we want that reference to be mutable
            //
            //  i.e. next is not Option<&mut Node<T>>, as we require in our
            //  definition of IterMut::next
            next: self.head.as_deref_mut(),
        }
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

// allow the use of:
//  let my_list: List<i32> = Default::default();
impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

// #Iterator 3 - we implement Iterator for IntoIter, IntoIter all
// the methods that are available to iterators
impl<T> Iterator for IntoIter<T> {
    // This is an associated type that Iterator expects us to define
    // This indicates to the implementation what the type inside the
    // iterator is
    // In the .next implementation, we return Option<Self::Item>, which
    // is also valid as Option<T>
    type Item = T;

    // next is the minimally required associated function we need to
    // implement
    fn next(&mut self) -> Option<Self::Item> {
        // First we access List using an index in our tuple struct,
        // then we .pop the value and return it
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    //fn next(&mut self) -> Option<Self::Item> {
    // The signature above is the desugared version of below.
    // Note how the lifetime of _self_ is different from the lifetime
    // used in the implementation
    //
    // i.e. after we get an iterator from the list, after using List.iter(),
    // that list_iterator.next() can be called multiple times because this
    // signature indicates that the lifetime of the iterator is independent
    // of the lifetime of the references being returned
    //
    // - we can have multiple immutable references at the same time
    fn next<'b>(&'b mut self) -> Option<&'a T> {
        self.next.map(|node| {
            //self.next = node.next.as_ref().map(|n| n.as_ref());
            self.next = node.next.as_deref();

            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    // For IterMut::next we need to return a mutable reference
    // on each iteration
    // A value may only have a single mutable reference at a time...
    // In order to support this, we need to ensure that for every iteration,
    // no one else may access the returned value
    // This can be achieved by using .take() to:
    //  - set self.next to None
    //  - mappin over the value we get from .take()
    //  - setting self.next to .next of its old value
    //  - returning a mutable reference to the value inside the old node
    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        self
            // this .next is IterMut::next
            .next
            // use Option::take to:
            //  - set self.next to None
            //  - allow for mapping over the contained value, which is Node<T>
            .take()
            // we now have Node<T>
            .map(|node| {
                // Set self.next to the next value of the node we got in the
                // closure by using Option::as_deref_mut, which:
                //  - uses deref coercion to convert Box<Node<T>> to Node<T>
                //  - returns Option<&mut Node<T>>
                self.next = node.next.as_deref_mut();

                // This return syntax seems unusual, but it's a more terse version
                // of creating a mutable reference using variable assignment:
                //
                // let x = &mut node.elem
                // return x
                //
                // On it's own, node.elem :: T
                //
                // We specify explicitly that we are returning a mutable
                // reference to the value
                //
                // - the closure receives &mut Node<T>
                // - node.elem returns T
                // - we return &mut T, as required by .next's signature
                &mut node.elem
            })
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        let mut xs = [0, 2, 3];

        xs.into_iter().for_each(|x| list.push(x));
        xs.reverse();

        let mut list_iter = list.into_iter();

        for x in xs {
            let value = list_iter.next();

            assert_eq!(value, Some(x));
        }
    }

    #[test]
    fn iter_mut() {
        let xs: Vec<i32> = (0..=3).collect();
        let mut list = List::new();

        xs.iter().for_each(|x| list.push(x));

        let mut mutable_iter = list.iter_mut();

        for (i, _) in xs.iter().enumerate() {
            let mut x = &xs[xs.len() - 1 - i];
            let z = mutable_iter.next();

            assert_eq!(z, Some(&mut x));
        }
    }
}
