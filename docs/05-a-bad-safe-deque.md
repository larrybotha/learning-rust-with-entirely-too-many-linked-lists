# A bad safe Deque

[fourth.rs](../src/fourth.rs)

The previous chapter introduced the interior mutability by explaining that it's
required for `Rc` and `Arc` to update their ref counts.

This chapter builds on that concept by _using_ interior mutability to build a
doubly-linked list.

From the tutorial:

> Disclaimer: this chapter is basically a demonstration that this is a very bad
> idea.

## `RefCell`

- `RefCell` enforces borrowing rules at runtime, instead of compile time:
  - if you violate one of the borrowing rules using `RefCell`, the `RefCell`
    will `panic!` and crash the application
- `RefCell::borrow` and `RefCell::borrow_mut` perform the same respective tasks
  as `&` and `&mut`

## The List

- we want a doubly-linked list, so we need:
  - a reference to the head
  - a reference to the tail
  - a representation of a node, which contains a value, and an optional
    reference to another node
  - a representation of a link between nodes
- each link must have the following features:

  - must be optional
  - must contain a node
  - the node must be able to be referenced multiple times, i.e. `Rc`
  - the node must be mutable so that we can change its value, i.e. `RefCell`

  We thus have the following type:

  ```rust
  type Link<T> = Option<Rc<RefCell<Node<T>>>>;
  //              [1]  [2]   [3]
  // 1 - optional - a node may or may not have a link to another, i.e.
  //    the tail or the head
  // 2 - we need to be able to reference it multiple times
  // 3 - we need to be able to mutate the node
  ```

- to add a new node at the head:
  - every node in the list should always have 2 references:
    - nodes at the tail and head:
      - the first node should have a reference to it from the head, and its
        successor node
      - the last node should have a reference to it from the tail and its
        predecessor node
    - every other node should have a reference to it from both its successor and
      predecessor
- to pop a value from the head:
  - take the current head
  - if the result is `None`, return that
  - otherwise we have a node, and:
    - we `.take` the next node
      - if that node is None, `.take` the tail to drop the ref
      - else:
        - `.take` `.prev` on that node to drop the ref
        - point the head to that node
    - return the node's element
- `Result::ok` converts from `Result<T, _>` to `Option<T>`
- `RefCell::into_inner` extracts the value from the cell, consuming the cell

### Drop

- we could do a complicated process of iterating through each value in the list
  while dropping, but it turns out we've already done all the cleaning up
  inside `List::pop_front` - we can leverage the associated function when
  implementing `Drop`:

  ```rust
  // ...
  fn drop(&mut self) {
      while self.pop_front().is_some() {}
  }
  // ...
  ```

- this is an interesting use of a while loop... relying on calling a function on
  a struct, and then evaluating that result to determine if the loop should be
  run again, without specifying a body

### Peek

- `RefCell::borrow` returns a `Ref<'b, T>`, not a reference to the underlying
  value

  - `Ref` implements `Deref`, so we can dereference the value inside of it,
    _BUT_... the lifespan of that value is tied to the lifespan of the
    `Ref`, not the `RefCell`
  - this means that if we attempt to return a reference to that value, we'll
    get a compiler error. e.g.

    ```rust
    use std::cell::Ref;

    let x = String::from("foo");
    let cell = RefCell::new(x);
    let mut y = &x;

    {
      let ref = cell.borrow(); // Ref<'b, String>
      y = *ref; // <- fails here
    } // ref dropped here, y would contain an invalid reference
    ```

- `Ref` is a functor, so it can be mapped over
- there's no pragmatic way of getting around the lifetime of the `Ref` and
  trying to return the reference inside the closure

  As a compromise, we can map over the `Ref`, extracting the value from the
  `Node`, and returning a reference to the element in the `Node`

- `Option::is_none` and `Option::is_some` are useful for determining whether or
  not an option contains anything
- it appears sufficient in Rust to evaluate that two values are equivalent in
  terms of their mutability, without having to evaluate that a value is
  actually mutable, e.g.:

  ```rust
  assert_eq!(&mut x, &mut y);

  //vs

  // bunch of code asserting that the mutated value of x is x'
  ```

### Iterator

#### IntoIter

- `IntoIter` is relatively straight-forward to implement because each item in
  the `List` will be consumed - we don't need to worry about managing
  references and using `Ref` as with
- `DoubleEndedIterator` allows for iterating backwards on types where `Iterator`
  has been implemented
  - only allowed for types that have a finite number of values, e.g. an infinite
    range would not be able to implement `DoubleEndedIterator`

#### Iter

- implementing `Iterator::iter` is going to be a mess, because we're back to
  dealing with returning `Ref<T>` and managing all of that... we skip the
  nightmare and let the implementation and consequences leaking implementation
  details of `peek_front` and `peek_back` speak for itself - this is not
  something one should implement for themselves
