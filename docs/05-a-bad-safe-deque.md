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
