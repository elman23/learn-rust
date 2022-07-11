use std::io;

fn main() {
	let mut name = String::new();
	let mut surname = String::new();
	let mut work = String::new();
	let mut age = String::new();
	
	println!("Inserisci il tuo nome: ");
	io::stdin().read_line(&mut name).unwrap();
	println!("Inserisci il tuo cognome: ");
	io::stdin().read_line(&mut surname).unwrap();
	println!("Inserisci il tuo lavoro: ");
	io::stdin().read_line(&mut work).unwrap();
	println!("Inserisci la tua età: ");
	io::stdin().read_line(&mut age).unwrap();

	println!("");
	println!("Dati anagrafici.");
	print!("Nome: {}", name);
	print!("Cognome: {}", surname);
	print!("Età: {}", age);
	print!("Lavoro: {}", work);
}
