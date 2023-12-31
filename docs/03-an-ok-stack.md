# An OK Stack

[second.rs](../src/second.rs)

## Option

- ok... `mem::replace` makes sense now in the context of the tutorial - it
  replaces an entry in memory with some other value, and returns the old
  value. When working with `Option`, and setting the value to `None`, we can
  either use `mem::replace`, or `Option::take` - they perform the same job:

  ```rust
  let mut x = Some("foo");
  let y = mem::replace(&mut x, None);

  assert_eq!(x, None);
  assert_eq!(y, Some("foo"));

  // or

  let mut x = Some("foo");
  let y = x.take(); // mutate x in-place

  assert_eq!(x, None);
  assert_eq!(y, Some("foo"));
  ```

- in addition to that little tidbit, matching on `Option` to transform
  `Some(x) => Some(y)` may also be more succinctly represented by
  `Option::map`:

  ```rust
  let mut x = Some(0);
  let y = match x {
     None => None,
     Some(x) => Some(x + 1)
  };

  // or...

  let mut x = Some(0);
  let y = x.map(|x| x + 1);
  ```

## Generics

- structs that use generics are always implemented with `impl<T>`:

  ```rust
  struct Foo<T> {
      value: T,
  }

  impl<T> Foo<T> {
      //...
  }
  ```

  We don't speak of only `Foo`, we now speak of `Foo` of `T` - `Foo` on its
  own doesn't paint the full picture of what we are working with

## Peek

- with the `peek` implementation, we can't use `.map` on `self.head` because:

  - we are returning `Option<&T>`, and...
  - `.map` takes ownership of the value

  We can't return a reference, because the closure in map will drop the value
  once it's done... this would leave us with an invalid reference - there
  would be no value to reference

  To address this, we use `Option::as_ref` which gives us an `Option` _to a
  reference *to a value*_, i.e. `Option<&T>`. With a _reference_ to a value, we
  can now safely return it without dropping the value in `self.head`

- deconstructing closure parameters affects what you can do with the values
  inside the closure. E.g. a value that is `&mut` is _only_ a value inside
  the function if we destructure it:

  ```rust
  struct Foo<T> {
    fn something_mut(&mut self) -> Option<&mut T> {
      // ...
    }
  }

  let mut f = Foo::new();

  // x cannot be mutated here, it is only T
  f.something_mut().map(|&mut x| {
    // do something with x
    // ...
  })

  // x can be mutated here, it is &mut T
  f.something_mut().map(|x| {
    // mutate x if you want
    // ...
  })
  ```

## IntoIter

- don't assume that `.for_each` exists on things that implement `.map` -
  `.for_each` is something specific to structs that implement the `Iterator`
  trait

  If something implements the `Iterator` trait, one is able to call either
  `.iter()` or `.into_iter()`, at which point `.for_each` and other `Iterator`
  methods would be available

- in [./src/second.rs](./src/second.rs) we implement `Iterator` for `List` by:
  - creating a tuple struct, `IntoIter`, which wraps `List`
  - adding an `.into_iter` associative function to `List` which returns an
    instance of `IntoIter` with the list wrapped inside - Rust complains
    about the `.into_iter` implementation, so there's likely a better way
  - we then implement `Iterator` for `IntoIter` - the only required method to
    implement is `.next`, and since getting the next item in `List` is done
    via `.pop`, we return the popped value inside `.next` on each iteration

## Lifetimes

- what are lifetimes:
  - they are names for regions / blocks / scopes in code
  - _references_ can be tagged with the names of these lifetimes
  - a reference associated with a lifetime must be valid for at least the
    duration of that lifetime - the underlying value may not be dropped until
    the lifetime the reference is associated with has first been dropped
- lifetimes are a set of constraints which ensure that references are around for
  at least as long as the regions of code in which they are found
- lifetimes are a mechanism to indicate to the _type system_ how long
  references need to be around... inferring lifetimes for functions is much
  easier, but we need to assist the compiler in describing how long references
  in types are around
- _lifetime elision_ is a set of rules in Rust that determine when lifetimes may
  be inferred, and thus not explicitly provided by the person writing the code

  e.g.

  ```rust
  // Lifetimes may be elided for functions with only a single input
  fn one_lifetime(x: &Foo) -> &Bar // is sugar for
  fn one_lifetime<'a>(x: &'a Foo) -> &'a Bar

  // Multiple inputs are assumed to have different lifetimes:
  fn do_something(x: &Foo, y: &Bar, z: &Quux) // is sugar for
  fn do_something<'a, 'b, 'c>(x: &'a Foo, y: &'b Bar, z: &'c Quux)

  // Methods assume that output lifetimes are that of _self_:
  fn my_method(&self, x: &Foo, y: &Bar) -> &Quux // is sugar for
  fn my_method<'a, 'b, 'c>(&'a self, x: &'b Foo, y: &'c Bar) -> &'a Quux
  ```

- lifetimes boil down to the following:
  > _the input must live at least as long as the output_ -
  > the compiler knows that the output might be around for a while after
  > the function has been called, and when it's safe to clean up, so is
  > the input
- `Option::as_deref` somewhat does the inverse of `Option::as_ref`

  - `Option::as_ref` is useful for getting a reference to a value inside an
    option, i.e.

    ```rust
    let x = Some(42);       // Option<i32>
    let ref_x = x.as_ref(); // Option<&i32>

    assert_eq!(ref_x, Some(&42));
    ```

    This allows us to pass references to values in `Option` around without
    moving the values

  - `Option::as_deref` works in a similar manner - it is a way of accessing a
    reference to the inner value of the `Option` BUT with the addition of
    dereferencing that value if it itself is a container, e.g. when using
    `std::boxed::Box`:

    ```rust
    let x = Some(Box::new(42));
    let ref_x = x.as_deref();

    assert_eq!(ref_x, Some(&42)); // as opposed to Box<Some<i32>>
    ```

- in places where a lifetime can be elided, but it may be useful to indicate
  that there is lifetime stuff going on, one can use the `'_` _explicitly
  elided lifetime_ syntax:

  ```rust
  impl<T> for Foo<T> {
    // lifetime is not needed here, as it's inferred, but we can indicate
    // that we've elided it
    fn do_the_thing(&self) -> SomeOtherThing<'_, T> {
      // ...
    }
  }
  ```

  This is syntactic sugar for:

  ```rust
  impl<T> for Foo<T> {
    fn do_the_thing<'b>(&'b self) -> SomeOtherThing<'a, T> {
      // ...
    }
  }
  ```

  The lifetime of Foo is independent of the lifetime of `SomeOtherThing` when
  calling `do_the_thing`.

  We can call `do_the_thing` how and when we want, and the compiler knows that
  `Foo` is not dependent on `Something` in any way

## std::Default

- implementing `Default` for a struct allows for default values of a struct to
  be generated:

  ```rust
  #[derive(Default)]
  struct Foo {
    x: i32,
  }

  let my_foo = Foo { ..Default::default() }; // or even
  let my_foo: Foo = Default::default();

  assert_eq!(my_foo.x, 0);
  ```

## IterMut

- we implement a mutable iterator in a similar manner to a non-mutable iterator:
  - create a struct called `IterMut` that contains a single field which holds
    a reference to the next node in the list
  - implement `Iterator` for `IterMut`
    - at a minimum we need to implement `Iterator::next`
    - `Iterator::next` needs to return `Option<&mut T>`
- with Rust's rules on mutable references, we may only have a single mutable
  reference to a value at a time. In order to ensure that there aren't
  multiple references to the value we return each time we call `.next`, we
  need to use `.take` when mapping over nodes in the list, that way:
  - the node is first assigned to `None` so that no other calls to `.next`
    anywhere else have access to the value
  - we then assign the next value inside the `.map` closure
