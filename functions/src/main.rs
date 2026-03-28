// Function fundamentals
// main is the entry point of the program
fn main() {
    println!("Hello, functions!");
    // ; means it is a statement otherwise it is an expression
    say_something_more();
    let sum_value = calculate_sum(2, 3);
    println!("Calculated sum is {sum_value}");
}

// this is a function used as a statement
// it performs an actions but doesn't reture any value
fn say_something_more() {
    println!("Something more to say");
}

// function signature consist of paramaters and return value if exits
// if no return value is defined the function is used in statements
// -> indictates the return value
fn calculate_sum(a: i32, b: i32) -> i32 {
    let result = a + b;
    // at the end an expression is expected because it is defined in the function signature
    // check out how ; is missing at the end
    result
}
