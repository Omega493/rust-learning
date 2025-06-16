fn main() {
    let x = 5; // Variable initialized. x has the value 5.
    let x = x + 1; // x now has the value 6. The `let` keyword is NECESSARY. Without it, we would get a commpile-time error.
    {
        let x = x * 2; // x is now 12. Again, the `let is necessary.
        println!("The value of x in the inner scope is {x}"); // Outputs 12.
    }
    // Scope of x = 12 ends here. x now has the value 6.
    println!("The value of x is {x}"); // Outputs 6.
}