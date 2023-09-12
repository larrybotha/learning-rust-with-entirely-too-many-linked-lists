# A Persistent Stack

[third.rs](../src/third.rs)

- for representing the following lists:

  ```
  list A ────┐
             │
        list B ── C ── D ── E
             │
  list X ────┘
  ```

  If we used `Box` to represent the shared references to `B`, we'd have an issue
  when it came to dropping the reference...

  If we drop `list X`... should that result in `list B` be dropped? With
  `Box` we'd expect that, because `Box` allows _exclusive ownership_, but
  then `list A` would no longer be pointing to the `list B`

  We can't use `Box` for managing lists in this way, because it doesn't allow
  for shared references

- `Rc` provides an alternative to `Box` - `Rc` allows for shared references via
  cloning; we can have multiple independent references to a value, and only
  when the count of those references reaches 0 will the `Rc` be dropped
- destructuring a reference when it's a function argument is is interchangeable
  with dereferencing the value inside the same function:

  ```rust
  let xs = [1,2,3];
  let mut ys: Vec<i32> = Vec::new();
  let mut zs: Vec<i32> = Vec::new();

  xs.iter().for_each(|x| {
    // dereference the value before appending
    ys.push(*x)
  })

  // destructure the reference at the argument-level
  xs.iter().for_each(|&x| {
    zs.push(x)
  })
  ```

- `.and_then` is syntactic sugar for removing nested `Option` or `Result`
  values:

  ```rust
  fn wrap(x: i32) -> Option<i32> {
    Some(x)
  }

  let x = Some(42); // Option<Option<i32>>
  let y = x.map(|v| wrap(v))  // Option<Option<i32>>
            .unwrap_or(None); // Option<i32>
  // or, more succinctly
  let y = x.and_then(|v| wrap(v)); // Option<i32>
  ```

## Implementing `Iterator`

- to implement `Iterator` for `List`, we must:
  - define a way to turn `List` into an iterator idiomatically, i.e. with a
    `.iter` method
  - `.iter` returns an object that upon iteration generates a reference to a
    value, whereas `.into_iter` returns an object that consumes the original
    value. We can't return a mutable reference, however, because the value
    returned on each iteration is dereferenced from `Rc`. `Rc` allows for
    multiple references, which in turn prevents us from making this references
    mutable (ignoring _interior mutability_ for the sake of this example)
  - what does adding an `.iter` method mean? It means we need to return
    something that implements `Iterator`. In that case, we create a struct
    with a single field
  - the field in our `Iter` is initialised with a reference to the value at the
    head of our list
  - we then need to implement the `Iterator` trait for our `Iter`, so that we
    have access to the methods that `Iterator` provides
  - we must implement an associated function on `Iter` called `.next` which returns an
    otion containing a reference to the value that `Iter` is currently
    holding
  - inside `.next` we:
    - map on the contained value, which is an `Option<Node<T>>`
    - if there is no node, the iterator returns `None`
    - if there is a node:
      - set the new value of `Iter` to the next value of the node
      - return a reference to the value in the node
- Rust allows for structs to both have fields and associated functions of the
  same name. This is why `Iter` can contain the next value in `.next`, while
  simultaneously implementing `.next()` for `Iterator`

## Implementing `Drop`

- as with `Box`, it appears that `Rc` seems to have a similar issue where if we
  drop an `Rc`, the contained value won't necessarily be dropped. This is
  because `Rc` contains a value that may have 0 or more references to it - if
  it were to drop the value the application would be in a state where it could
  have invalid references - something Rust disallows by default
- what we can do, however, is leverage a feature of `Rc` - `Rc::try_unwrap`
  exists to 'release' the value from the `Rc` _if_ there was only 1 reference
  to the value
- `Rc::try_unwrap` returns a `Result`
  - if there is only 1 reference:
    - you'll get an `Ok(x)`
    - the reference count will be decremented to 0
    - the `Rc` will be dropped
  - otherwise `Err`
- for `Drop`, we can match on `Ok`, which allows us to operate on the node
  contained in the `Rc`
- where do we start...? Looking at the diagram above, if we were to drop `List A`
  starting from the head, and using `Rc::try_unwrap`, we would be able to safely
  drop the first item, but we'd then be unable to drop the second item. At this
  point we can stop, at which point Rust can drop the list, and drop the
  reference to `List B`

  Now, if wen wanted to drop `List X`, the first item would be safely dropped,
  and because `List B` now only has 1 reference, it can be safely dropped, and
  we can traverse the remainder of the list safely dropping values as we go

## Arc

- `Rc` is not thread safe - if a value is shared across threads, the value could
  be dropped or mutated in one thread, and the other thread wouldn't know
  about it
- to address this `Arc` is available, which is drop-in and thread-safe
  alternative. Thread-safety comes with overhead, so only use it if you need
  it

## Send and Sync

- there are two major classes of values that allow for interior mutability:

  - cells - single-threaded
  - locks - multi-threaded

  atomics are primitives that behave like locks

- thread-safety is modeled with two traits in Rust:
  - `Send`
    - values are `Send` if they are safe to _move_ between threads
  - `Sync`
    - values are `Sync` if they are safe to _share_ between threads
- as with `Copy`, `Send` and `Sync` are automatically derived for types where
  all of their members are `Send` or `Sync`
- most types are `Send` and `Sync` but types that use _interior mutability_ are
  not! Both `Rc` and `Arc` use interior mutability
  - the reference counting in `Rc` and `Arc` requires interior mutability
  - `Rc` achieves reference counting using cells - cells are not thread-safe
  - `Arc` achieves reference counting using atomics - atomics _are_ thread-safe,
    so despite interior mutability, `Arc` _is_ thread safe because it
    implements interior mutability using thread-safe types
