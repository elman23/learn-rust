use std::io;

fn main() {
	let mut input = String::new();

	println!("Inserisci un numero: ");

	io::stdin().read_line(&mut input).unwrap();

	let n: i32 = input.trim().parse().unwrap();

	println!("Il numero inserito è: {}", n);
}
