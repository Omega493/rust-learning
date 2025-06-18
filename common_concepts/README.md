# Rust Learning Notes

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
- Since `x` isn't a mutable variable, this would throw an error.
- However, `let mut x = 5` won't cause this issue.

## Constants
- Constants are not only immutable by default, but also are always immutable (adding the `mut` keyword would do nothing.)
- Can be declared in any scope, even global scope (outside of every function)
- Consider `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`. Rust is able to evaluate a limited set of operations at compile. So rather than using 10800, we can use 60 * 60 * 3.
- Constants are valid for the entire time a program runs, within the scope in which they were declared.

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

## Datatypes
- Like other languages, every variable in Rust has some datatype. Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases where many types are possible, we have to tell the compiler what datatype to use, as we saw in [Guess the Number game](../guess_the_number/src/main.rs).
- Consider the following:-
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
If we don't add the `: u32` type annotation, Rust will display an error.
### Scalar types
A "scalar" type represents a single value. Rust has four primary scalar types - integer, floats, booleans and characters.
#### Integers
| Length | Singed | Unsigned |
|--------|--------|----------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |
- Signed variants can store numbers from -2<sup>n-1</sup> to 2<sup>n-1</sup>-1. Example: `i8` can store numbers from -2<sup>7</sup> to 2<sup>7</sup>-1, or -128 to 127.
- Unsigned variants can store numbers from 0 to 2<sup>n</sup>-1. Example: `u8` can store numbers from 0 to 2<sup>8</sup>-1, or 0 to 255.
- `isize` and `usize` depend on the architecture of the computer a program is running on. For a 32-bit system, its 32-bits and for 64-bit system, its 64-bit.
##### Integer Overflow
- Let's say a variable is marked `u8`, which means it can hold values from 0 to 255. If we however put, let's say, 256, integer overflow would occur. 
#### Floating-Point types
- Rust has two floating-point  numbers, `f32` and `f64`. Default is `f64`.
### Compound Types
A compound type can group multiple datatypes into one variable. Examples are tuples and arrays.
#### Tuples
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
```rust
let tup = (493, 69.42, 'c');
let (i, j, k) = tup;
println!("The value of 'j' is {j}"); // Outputs 69.42
```
Here, what we do in line 2 is called "destructuring", meaning, separating the tuple `tup` into three parts. We can also use indexing:
```rust
let third_value = tup.2; // Index of third value in the tuple tup
println!("The third value is {third_value}"); // Outputs c
```
#### Arrays
Every element of an array must have the same type. Arrays also have a fixed length.
```rust
let a: [i35; 5] = [1, 2, 3, 4, 5];
```
