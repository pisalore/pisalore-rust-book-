const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn another_function(x: i32, character: char) {
    println!("Another function called. The values of x is {x}, that is associated with the {character} char.");
}

fn sum(x: f64, y: f64) -> f64 {
    return x + y;
}

fn collection_looping(list: &[i32]) {
    let mut index = 0;

    while index < list.len() {
        println!("the value is: {}", list[index]);

        index += 1;
    }
}

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

    let mut t: u128 = 100;
    t += 200;
    println!("{t}");

    // Functions
    another_function(1, 'a');
    another_function(2, 'b');
    another_function(3, 'c');

    // Difference between statement and epression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    println!("The result for sum is: {}", sum(10.0, 476.43));

    // Control flow
    let number = 1;
    let check_number = 1000;
    if number > check_number {
        println!("{} is greater than 100", number)
    } else {
        println!("{} is less than 100", number)
    }

    // Ternary operator
    let condition = true;
    let number = if condition {
        let x = 10;
        x * 10 - 7
    } else {
        let x = 10;
        x * 10 + 7
    };

    println!("The value of number is: {number}");

    // Looping
    let a = [1, 2, 3, 4, 5, 6];
    let b = [3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5];

    collection_looping(&a);
    collection_looping(&b)
}
