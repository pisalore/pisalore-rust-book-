const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Costants
    println!("Use a constant: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        // Inner scope of main function
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    let spaces = "   ";
    println!("spaces variable before... '{spaces}'");
    let spaces = spaces.len();
    println!("{spaces} and after.");

    // Data types
    let _guess: u32 = "42".parse().expect("Not a number!");
}
