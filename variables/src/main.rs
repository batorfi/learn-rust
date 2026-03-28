fn main() {
    //x must be mutable to be reassigned
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    // tuple is a compound type that can hold multiple values of different types
    let tup: (u32, char, i64) = (5, '@', 6);
    println!("tup variable value = {:?}", tup);
    // tuple values can be destructured into separate variables
    let (x, a, y) = tup;
    println!("x = {}, a = {}, y = {}", x, a, y);
    // individual tuple values can be accessed by 0 based index
    let at_sign = tup.1;
    println!("at_sign = {}", at_sign);
    // array is a compound type that can hold multiple values of the same type
    // arrays are allocated on the stack and have a fixed size
    // the type is specified as [type; size]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr values = {:?}", arr);
    println!("First element = {}", arr[0]);
    // pre-filled array is an array with all elements initialized to the same value
    // the type is specified as [type; size]
    // semi-colon is used to separate the type and size
    let pre_filled_array: [i32; 5] = [0; 5];
    println!("pre_filled_array = {:?}", pre_filled_array);
}
