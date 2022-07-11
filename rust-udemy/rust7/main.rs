fn main() {

	let sigla = "IT";
	
	let stato = match sigla {
		"IT" => { println!("Trovato!"); "Italia" },
		"EN" => { println!("Trovato!"); "Inghilterra" },
		"FR" => { println!("Trovato!"); "Francia" },
		_ => "sconosciuto",
	};

	println!("Lo stato è {}.", stato);
}
