## Chapter 1

### Hello Cargo

- Rust is an _ahead-of-time compiled language_, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- `rustc main.rs` allows to compile the Rust file
- `Cargo` is the rust ackage manager.
- `cargo new project` create a new project called `project`, adding some stuff such as git ant a `toml` file where dependencies are defined.
- Cargo expects source files to live inside the `src` directory.
- `cargo build` is the commands that builds the sources. Th executable is stored inside a `target/debug` directory.
- Using `cargo run` it is possible to run the executable.
- `cargo check` performs code checking, i.e. verifies that everything compiles.
- `cargo build --release` build rust programs for releases, that is, using optimizations.
- The true power of `¢argo` approach is that you can simply run the following using every rust project:

```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```
