# day

Separate crate for `day!()` procedural macro, which strips out the day
number from a filename. For example, in a file named `path/to/day01.rs`,

```rust
  let d: &str = day!();
```

expands to

```rust
  let d: &str = "01";
```

This is used in the `check!()` macro to find a solution's `input` and
`example` files.
