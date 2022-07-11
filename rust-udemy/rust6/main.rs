fn main() {
	let a = 10;
	let b = 20;
	let c = 30;
	let mybool = true;

	let bool1 = a > 1;
	println!("Il primo booleano è: {}", bool1);
	let bool2 = b != 20;
	println!("Il secondo booleano è: {}", bool2);

	if mybool {
		if a == 10 && b == 20 {
			println!("a è 10 e b è 20");
		} else {
			println!("a non è 10 oppure b non è 20");
		}
	}
}
