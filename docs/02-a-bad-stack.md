# A Bad Stack

[first.rs](../src/first.rs)

- `mem::replace(&mut something, something_else)` - what is this...? O.o
  What `mem::replace` does is:

  - replaces the mutable value `something` with `something_else`
  - returns the original value of `something`

  Soooo...

  ```rust
  use std::mem;

  let mut x = String::from("foo");
  let y = mem::replace(&mut x, String::from("bar"));

  assert_eq!("x", String::from("bar"));
  assert_eq!("y", String::from("foo"));
  ```

- with recursive objects in environments where memory is not managed for us, care
  must be taken to ensure that memory is properly cleaned up. In the case of
  our first attempt at building the stack, if we had to drop the `List` and
  depend on Rust's dropping mechanism, we may end up in a situation where the
  stack will be blown if the recursive data structure is large enough.

  This is because we can't rely on tail recursion for all items in the data
  structure to be dropped, because at the point where the `Box` is dropped,
  there's no mechanism that first drops the `Node` it contains... we'd be left
  with a bunch of dangling `Node` entities, which Rust would presumably clean
  up if the stack is not blown before then.

  e.g.:

  ```
  List = Node A -> Node B -> ...

  drop List -> executes drop(Link)
    drop Link -> executes drop(Box)
      drop Box for Node A -> does not execute drop(Node)!
        !!! tail recursion broken at this point !!!
        Box is dropped, Node is not, and is left dangling
  ```

  To account for this, we need to _manually_ implement `Drop` for List, where:

  - initialise the current node while setting `self.head` to `Link::Empty` using
    `mem::replace`
  - while we have a current node
    - use `mem::replace` to get the value of `Node.next` while setting
      `Node.next` to `Link::Empty`
    - perform the same operation on the new value until `Node.next` is no
      longer `Link::More`

- `#[cfg(test)]` indicates to the Rust compiler that the code inside the module
  should only be compiled during testing
- tests are generally written inline in Rust, and inside a non-public module
  called `test`:

  ```rust
  #[cfg(test)]
  mod test {
      // ...
  }
  ```

- Adding the `#[test]` attribute to a function will run it during testing. This
  is useful to differentiate between functions used for setup and functions
  used for running actual tests
