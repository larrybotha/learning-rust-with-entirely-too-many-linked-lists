# An OK Unsafe Queue

[fifth.rs](../src/fifth.rs)

This is as far as this journey is going to - learning about raw pointers and
unsafe Rust is not valuable to me right now. TBC... maybe

## Takeaways

- in a singly linked list, implementing a stack and a queue can be done in
  multiple ways by placing the `push` and `pop` methods at either bound of the
  list:
  - Stack:
    - `push` and `pop` at head:
      - `push` is O(1)
      - `pop` is O(1)
    - `push` and `pop` at tail:
      - `push` is O(1) - go to the tail, make the new node the new tail
      - `pop` is O(n) - one would need to traverse the entire list to get to the
        nth-1 item in order to pop
  - Queue:
    - `push` at head, `pop` at tail
      - `push` is O(1)
      - `pop` is O(n) (see above)
    * `push` at tail, `pop` at head
      - `push` is O(1)
      - `pop` is O(1)
- `Box`, which is a smart pointer, i.e. a special type of _reference_, owns the
  value that it holds, and does not implement `Copy`. This has a number of
  implications:
  - passing a `Box` around will _move_ it - it cannot be moved into two places
    in the same scope
  - when the `Box` is dropped, the value it owns is dropped
- unsafe Rust is a superset of Rust that allows you to do everything
- _raw pointers_ are a feature of unsafe Rust. Raw pointers are like the wild
  west:
  - Rust's borrowing rules don't apply
  - types can be changed via casting
  - mutability can be changed via casting
  - pointers can dangle, be null, be misaligned, or point to uninitialised
    memory

### Implementation attempt 1

[fifth_attempt1.rs](../src/fifth_attempt_1.rs)

- we attempt to use `Box` as our tail value. This results in a few problems:
  - because `Box` is not `Copy`, we cannot assign its value to both `self.tail`
    and `n_minus_1th_node.next` at the same time
- if the compiler let us do this, the `Box`ed value would be double-freed:
  - once at `self.tail`
  - again at `n_minus_1th_node.next`
- additionally, by replacing / extracting the current `Box`ed value out of
  `self.tail`, and not assigning that value to anything, the `Box`ed value
  will be dropped on _every_ call to `List::push`
- so... what can we do...? `Box` seemed convenient, because it's a smart
  pointer, but ownership is a problem. Well... _references_ are pointers too,
  and they don't come with the ownership complexity that we have with `Box` in
  this implementation

  We can assign references in multiple locations as long as we follow Rust's
  rules for borrowing. Onwards to
  [Implementation attempt 2](#implementation-attempt-2)

### Implementation attempt 2

- the problem in [Implementation attempt 1](#implementation-attempt-1) was that
  we defined `self.tail` as a `Box`:
  - `Box` is not `Copy`, i.e. it can't be used both as `self.tail` and
    `node.next`
- to address this, we instead indicate that `self.tail` is a mutable reference
  - we want to be able to have multiple references to the node, from `self.tail`
    and from `node.next`
  - we want to be able to mutate that value when a new value is passed in, i.e.
    we need to set `.next` on the old tail when a new tail is pushed
  - we don't want to go through the nightmare of using `Rc`, as we demonstrated
    in [./04-a-persistent-stack.md](./04-a-persistent-stack.md) and
    [./05-a-bad-safe-deque.md](./05-a-bad-safe-deque.md)
- structs that contain fields that are references require lifetime parameters to
  be provided - these parameters indicate to the compiler that the values of
  referenced fields in the struct must live for at least as long as the
  instance itself
  - if we dropped the referenced value _before_ dropping the reference, we'd
    be left with a reference to a location in memory that no longer holds
    what we expect - a dangling reference
- in our `.push` implementation we do the following:

  - use `self.tail.take()` to get the current tail to:
    - extract the current value out of the tail
    - set the tail to None, temporarily
  - match on the current / now old tail:

    - if we have `Some`, then we know that the old tail contained a value
      - set `.next` of this old value to the value of the new node
    - otherwise there was no tail, and this new value will be the new tail
      - set `self.head` to the new value
    - for both branches, we need to return a mutable reference to the new value.
      `Box` has one level of indirection that we can remove by dereferencing
      a. `Option` allows us to remove this indirection by using
      `Option::as_deref` and `Option::as_deref_mut`, which does the following:

      ```rust
      Option::as_deref_mut :: Option<T> -> Option<&mut Target::T>
      ```

  - now that we have a mutable reference to the value, we can assign `self.tail`
    to this reference
