## Chapter 3

### Programming concepts

#### Variables and mutability

- Variables _immutability_. To define a mutable variable we must use the `mut` keyword.
- We can use _constants_. Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
- Shadowing: we can declare a variable and use another variable with the same name. Shadowed variables can be different in differnt scopes. Using shadowing is permitted only if we use the `let` keyword. This is an important difference with the `mut` usage.

#### Data types

---

#### Functions

- `fn` keyword to define a function:

  ```
  fn main() {
      println!("Hello, world!");

      another_function();
  }

  fn another_function() {
      println!("Another function.");
  }
  ```

- Function definition ordering does not matter to rust compiler.
- Function parameters must have a type.
- Function bodies are made up of a series of statements optionally ending in an expression
- Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
- A new scope block created with curly brackets is an expression
- We can write functions with return values simply evaluating the last expression of the function or using the `return` keyword.
- The main difference between statemenet and expression is that the statement does not return anything, while expression does.

#### Control flow

- Rust does not perform automatic conversion of non boolean variables when evaluating conditions. >We must eplicit boolean expression along with if clauses.
