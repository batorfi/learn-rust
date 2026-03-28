// Ownership is a set of rules how a Rust program manages memory. Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
// In order to understand ownership one must understand the stack and the heap.
// All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses.
// Basic rules:
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    // s is a string literal stored in the stack
    // it has known size both at compile and runtime
    let s = "hello";
    println!("{}", s);
    {
        // when v goes out of scope it will be dropped
        let v = "new content";
        println!("{}", v);
    }

    // Now let's define a variable stored in the heap
    // length of s can change any time in the future
    // the memory is requested from the memory allocator at runtime
    let mut s = String::from("New hello");
    // we can append some additional text, the memory allocator needs to add more space to store the additional characters
    s.push_str(", something added to it");

    {
        let v = String::from("Another text");
        println!("{}", v);
    }
    // when v goes out of scope the allocator drops it.
    // It means it will call the drop() methon on the instance

    // Now we "move s into s2", after movement s is gone
    let mut s2 = s;
    // We cannot use s anyomore only s2 avaliable here
    println!("{}", s2);

    //We can force to keep the original variable by cloning
    let s3 = s2.clone();
    let s4 = s3.clone();
    println!("{}", s2);
    println!("{}", s3);

    // when function called with a variable stored on the heap
    // the function takes the ownership and the variables passed
    // goes out of scope
    new_owner(s3);
    // cannot to this here: println!("{}", s3);

    let length = calculate_length(&s2);
    println!("Length: {length}");

    change_content(&mut s2);
    println!("{}", s2);

    let r1 = &s4;
    let r2 = &s4;

    println!("{r1}, {r2}");
}

fn new_owner(s: String) {
    println!("{}", s);
}

// References and borrowing
// To avoid taking ownership when calling a function with a parameter
// we can use a reference to borrow the value for processing
// "&" indicated the we refernce a String value
// We call the action of creating a reference borrowing.

fn calculate_length(s: &String) -> usize {
    s.len()
}

// We cannot change a referrenced value unless we indicate that it is mutated

fn change_content(s: &mut String) {
    s.push_str(" ,added more content");
}

// Slices are another type of refeernces
