fn main() {
    
    // owner of the value is the variable name hello
    let hello: Vec<i32> = (0..10).collect();

    fn do_stuff(val: &Vec<i32>) {
        println!("{}", val.len());
    }

    // do_stuff borrows the value from hello
    do_stuff(&hello);
}
