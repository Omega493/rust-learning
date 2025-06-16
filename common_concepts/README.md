# Rust Learning Notes

---

## Variables
- Rust has exclusive specification of mutable variables
- The following code will throw an error:-
```rust
fn main() {
let x = 5; // First assignment to x
println!("The value of x is {x}");
x = 6; // Second assignment to x
println!("The value of x is {x}");
}
```
- Since 'x' isn't a mutable variable, this would throw an error.
- However, `let mut x = 5` won't cause this issue.

---

## Constants
- Constants are not only immutable by default, but also are always immutable (adding the `mut` keyword would do nothing.)
- Can be declared in any scope, even global scope (outside of every function)
- const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
- Rust is able to evaluate a limited set of operations at compile. So rather than using 10800, we can use 60*60*3.
- Constants are valid for the entire time a program runs, within the scope in which they were declared.

---

## Shadowing 
- We can declare a new variable using the name of an existing variable.  When this happens, the second variable is what the compiler will see when the variable name is used. The second variable overshadows the first, taking any further uses of the variable name onto itself until either it itself is shadowed, or the scope ends. See [main.rs](src/main.rs).
- Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name. This has been demonstrated while creating the [Guess the Number game](../guess_the_number/src/main.rs).
- Another demonstration:
Say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:
```rust
let spaces = "   ";
let spaces = spaces.len();
```
The first `spaces` variable is a string type and the second `spaces` variable is a number type. Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; instead, we can reuse the simpler `spaces` name. However, if we try to use `mut` for this, as shown here, we’ll get a compile-time error:
```rust
let mut spaces = "   ";
spaces = spaces.len();
```

---
