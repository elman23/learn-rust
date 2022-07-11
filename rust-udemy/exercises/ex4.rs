use std::io;

fn main() {
	let mut n1 = String::new();
	let mut n2 = String::new();
	let mut operatore = String::new();

	println!("Inserisci un operatore matematico: ");
	io::stdin().read_line(&mut operatore).unwrap();

	println!("Inserisci il primo numero: ");
	io::stdin().read_line(&mut n1).unwrap();
	
	println!("Inserisci il secondo numero: ");
	io::stdin().read_line(&mut n2).unwrap();

	let num1: i32 = n1.trim().parse().unwrap();
	let num2: i32 = n2.trim().parse().unwrap();

	let risultato = match operatore.trim() {
		"+" => println!("La somma è {}", num1 + num2),
		"-" => println!("La differenza è {}", num1 - num2),
		"*" => println!("Il prodotto è {}", num1 * num2),
		"/" => println!("Il quoziente è {}", num1 / num2),
		_ => println!("Operatore non riconosciuto."),
	};
}
