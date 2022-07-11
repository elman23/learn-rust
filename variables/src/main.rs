use std::io;

fn main() {
    let x = 5;

    let x = x + 1; // shadowing number one
     
    {
        let x = x * 2; // shadowing number two
        println!("Value of x: {x}");
    }

    println!("Value of x: {x}"); // what is x right now?

    // MORE ON SHADOWING

    //let spaces = "   ";

    // This works
    //let spaces = spaces.len();

    // This doesn't work
    //spaces = spaces.len();

    let x: u32 = 234;
    println!("The value of x is: {x}");

    let y: u8 = b'A';
    println!("The value of y is: {y}");

    let z: f32 = 43.12;
    println!("The value of z is: {z}");

    // addition
    let sum = 5 + 3;
    println!("Sum: {sum}");

    // subtraction
    let difference = 90.3 - 54.8;
    println!("Difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0
    println!("Quotient: {quotient}");
    println!("Floored: {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");

    let t = true;
    println!("Boolean t: {t}");

    let f: bool = false;
    println!("Boolean f: {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Char c: {c}");
    println!("Char z: {z}");
    println!("Char heart_eyed_cat: {heart_eyed_cat}");

    //
    // Compound types (tuples, arrays)
    //
    
    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let first = tup.0;
    println!("First element of the tuple is: {first}");

    // ARRAYS
    let a = [1, 2, 3, 4, 5]; // data allocated on the stack rather than the heap
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let second = a[1];
    println!("The value of the second entry is {second}");

    // repeat a value for a number of times
    let b = [3; 5];
    let third = b[2];
    println!("The value of the third entry is {third}");

    test_array();
}

fn test_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please, enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    // Following can result in runtime error at the point of using an 
    // invalid value in the indexing operation.
    // This is an example of Rust’s memory safety principles in action. 
    // In many low-level languages, this kind of check is not done, 
    // and when you provide an incorrect index, invalid memory can be accessed. 
    // Rust protects you against this kind of error by immediately exiting 
    // instead of allowing the memory access and continuing.
    let element = a[index];

    println!("Value of the elementat index {index}: {element}");
}