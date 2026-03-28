// using the io crate from the standard library
use rand::Rng; //Rng is a trait that provides random number generation methods
use std::cmp::Ordering; //cmp is a module responsible for comparison operations, Ordering is an enum representing the result of a comparison
use std::io; //std = module responsible for input/output operations, io = trait, stdin = functionk

// main function is the entry point of the program
fn main() {
    println!("Guess the number!");

    // range is defined as 1..=100 means 1 to 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // creates a mutable variable guess of type String
        // String::new() returns a new instance of String
        // String is always avalilable, no need to import it because it's in the prelude
        let mut guess = String::new();

        // :: is the namespace resolution operator, used to access items in a module
        // stdin() is an associated function on the std::io::Stdin type
        // it is defined in the std::io module as part of the public API
        // associated functions are available without creating an instance of the type
        // io::stdin() returns a std::io::Stdin instance, which has a read_line method
        io::stdin()
            // read_line returns a Result, which is an enum with Ok and Err variants
            // Ok contains the number of bytes read, Err contains an io::Error
            .read_line(&mut guess) //& = reference, read_line modifies guess in place
            // expect is a method on Result that panics if the Result is Err
            // panics in Rust means the program will terminate with an error message
            .expect("Failed to read line, program terminates now...");

        // trim removes whitespace from the beginning and end of a string
        // parse converts a string to a number, returning a Result
        // expect is a method on Result that panics if the Result is Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a wildcard pattern that matches any value
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // cmp is a method on String because it implements the Ord trait
        // it compares two strings lexicographically
        // returns an Ordering enum variant (Less, Greater, or Equal)
        match guess.cmp(&secret_number) {
            //match expressions are made up of arms, each with a pattern and a body
            // each arm is checked in order, and the first one that matches is executed
            // => is the pattern-matching operator, separating the pattern from the body
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            // we can use a block to execute multiple statements in an arm
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}
