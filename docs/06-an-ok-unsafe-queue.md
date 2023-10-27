# An OK Unsafe Queue

[fifth.rs](../src/fifth.rs)

## Takeaways

- unsafe Rust is a superset of Rust that allows you to do everything
- _raw pointers_ are a feature of unsafe Rust. Raw pointers are like the wild
  west:
  - Rust's borrowing rules don't apply
  - types can be changed via casting
  - mutability can be changed via casting
  - pointers can dangle, be null, be misaligned, or point to uninitialised
    memory
