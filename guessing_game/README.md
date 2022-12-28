## Chapter 2

### Guessing game

- `use` directive allows to _use_ libraries.
- `let` allows to define variables. **NB: variables are immutable**. To make a variable mutable we must use the `mut` keyword:

```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

- The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- A `crate` is a collection of Rust source code files. External crates to be used in our project are defined in `Cargo.toml` file.
- `Cargo.lock` file ensures reproducible builds
- `cargo doc --open` generates documentation for all the installed crates.
- Rust has a **strong**, static type system.
- Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables.
- Result is an enum that has the variants Ok and Err
  `match` expressions allows us to to verify matching results and control our code flow.
