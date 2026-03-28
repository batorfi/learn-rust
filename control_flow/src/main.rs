fn main() {
    println!("Hello, control flow!");
    let a: i32 = 42;
    // if expects a boolean expression
    if a < 42 {
        println!("Variable a is less then 42");
    } else if a > 42 {
        println!("Variable a is greater than or equal to 42");
    } else {
        println!("Variable a is equal to 40");
    }

    loop_until_ten();

    iterate_while_ten();

    iterate_with_range();
}

fn loop_until_ten() {
    println!("Looping until 10");
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        } else {
            println!("{i}");
        }
        i += 1;
    }
}

fn iterate_while_ten() {
    println!("Iterating while 10");
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}

// this is the most readable and simplest approach
// .. defines a range that is enumerable as a collection of numbers
fn iterate_with_range() {
    println!("Iterate with for and range");
    for i in 0..9 {
        println!("{}", i);
    }
}
