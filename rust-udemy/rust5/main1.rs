use std::io;

fn main() {
	let mut input = String::new();

	println!("Inserisci una parola: ");

	io::stdin().read_line(&mut input).unwrap();

	println!("La parola inserita è: {}", input);
}
