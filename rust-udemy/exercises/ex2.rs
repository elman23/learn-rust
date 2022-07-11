use std::io;

fn somma(n1: i32, n2: i32) -> i32 {
	n1 + n2
}

fn differenza(n1: i32, n2: i32) -> i32 {
	n1 - n2
}

fn prodotto(n1: i32, n2: i32) -> i32 {
	n1 * n2
}

fn quoziente(n1: i32, n2: i32) -> i32 {
	n1 / n2
}

fn resto(n1: i32, n2: i32) -> i32 {
	n1 % n2
}

fn main() {
	let mut n1 = String::new();
	let mut n2 = String::new();

	println!("Inserisci il primo numero: ");
	io::stdin().read_line(&mut n1).unwrap();
	let num1: i32 = n1.trim().parse().unwrap();

	println!("Inserisci il secondo numero: ");
	io::stdin().read_line(&mut n2).unwrap();
	let num2: i32 = n2.trim().parse().unwrap();

	println!("Somma: {}", somma(num1, num2));
	println!("Differenza: {}", differenza(num1, num2));
	println!("Prodotto: {}", prodotto(num1, num2));
	println!("Quoziente: {}", quoziente(num1, num2));
	println!("Resto: {}", resto(num1, num2));
}
