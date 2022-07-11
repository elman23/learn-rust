fn main() {
    let number = 3;

    if number < 5 {
        println!("The number is less than five!");
    } else {
        println!("The number is more than five!");
    }

    // the condition 
    // if number { ... }
    // doesn't work: must be bool

    if number != 0 {
        println!("The number is not zero!");
    }

    if number % 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 3 == 0 {
        println!("Number divisible by 3");
    } else {
        println!("Number not divisible by 3, 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number is {number}");

    // not allowed:
    //let number = if condition { 5 } else { "six" };

    // infinite loop
    //loop {
    //  println!("again");
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("The End! count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("element = {element}");
    }

    for number in (1..4).rev() {
        println!("number = {number}");
    }
    println!("LIFTOFF!");
}
