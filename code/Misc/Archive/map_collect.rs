fn main () {
	let names: Vec<&str> = vec![
		"Kenneth",
		"Philip",
		"Linda",
		"Giulia"
	];
	/* 
	Applico .map() a .iter(), nella sua firma indico `s: &str` in forma implicita e
	uso .collect() per convertire l'iteratore risultante in un Vec<String>, 
    raccogliendo tutti i nomi trasformati in maiuscolo in un vettore.
	*/
	let uppercase_names: Vec<String> = names.iter().map(|s|-> String {s.to_uppercase()})
                                      .collect();
	
    println!("{:?}", uppercase_names);
}