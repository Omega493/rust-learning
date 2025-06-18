fn main() {
    let x = 5; // Variable initialized. x has the value 5.
    let x = x + 1; // x now has the value 6. The `let` keyword is NECESSARY. Without it, we would get a commpile-time error.
    {
        let x = x * 2; // x is now 12. Again, the `let is necessary.
        println!("The value of x in the inner scope is {x}"); // Outputs 12.
    }
    // Scope of x = 12 ends here. x now has the value 6.
    println!("The value of x is {x}"); // Outputs 6.

    let tup = (493, 69.42, 'c');
    
    let (_i, j, _k) = tup; // "_i" and "_k" indicates unused variable.
    println!("The value of 'j' is {j}"); // Outputs 69.42
    
    let third_value = tup.2; // Index of third value in the tuple tup
    println!("The third value is {third_value}"); // Outputs c
}