fn main() {
    println!("Hello, world!");

    another_function();

    third_function(5);

    print_labeled_measurement(5, 'h');

    statement_expression();

    let f = five();
    println!("Value: {f}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function running!");
}

/**
 * We can define functions to have parameters, 
 * which are special variables that are part of
 * a function’s signature. When a function 
 * has parameters, you can provide it with 
 * concrete values for those parameters. 
 * Technically, the concrete values are called 
 * arguments, but in casual conversation, 
 * people tend to use the words parameter and 
 * argument interchangeably for either the 
 * variables in a function’s definition or the 
 * concrete values passed in when you call a function.
 */

fn third_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/**
 * Function bodies are made up of a series of statements 
 * optionally ending in an expression. So far, the functions 
 * we’ve covered haven’t included an ending expression, but 
 * you have seen an expression as part of a statement. 
 * Because Rust is an expression-based language, this is an 
 * important distinction to understand. Other languages don’t 
 * have the same distinctions, so let’s look at what statements 
 * and expressions are and how their differences affect the bodies of functions.
 * Statements are instructions that perform some action and 
 * do not return a value. Expressions evaluate to a resulting value. 
 * Let’s look at some examples.
*/
fn statement_expression() {
    
    // statement
    let y = 5;
    println!("Value is {y}");

    // not good
    //let x = (let y = 6);
    // The let y = 6 statement does not return a value, so there isn’t 
    // anything for x to bind to. This is different from what happens in 
    // other languages, such as C and Ruby, where the assignment returns 
    // the value of the assignment. In those languages, you can write 
    // x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

    // Expressions evaluate to a value and make up most of the rest of the 
    // code that you’ll write in Rust. Consider a math operation, such as 5 + 6, 
    // which is an expression that evaluates to the value 11. Expressions 
    // can be part of statements: in Listing 3-1, the 6 in the statement 
    // let y = 6; is an expression that evaluates to the value 6. Calling a 
    // function is an expression. Calling a macro is an expression. A new scope 
    // block created with curly brackets is an expression, for example:
    let y = {
        let x = 3;
        x + 1 // no semicolon here!
    };
    // Expressions do not include ending semicolons. 
    // If you add a semicolon to the end of an expression, 
    // you turn it into a statement, and it will then not return a value.
    println!("The value is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}