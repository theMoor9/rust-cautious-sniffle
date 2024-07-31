fn main () {
	let animals: Vec<String> = vec![
		"Cat".to_owned(),
		"Lion".to_owned(),
		"Dog".to_owned(),
		"Wolf".to_owned(),
		"Shark".to_owned(),
		"Cat".to_owned(),

	] ;

	let numbers: Vec<i32> = vec![
		0,
		3,
		-12,
		6,
		-9,
		96,
	];
	
	// Soluzzione con ciclo for
	/*
	let mut veterinary_list: Vec<String> = vec![] ;
	for anml in animals {
		healedanimal(anml)
		veterinary_list.push(String::from(anml) + " Rescued!")
	}
	*/
	
	// Soluzione con iteratore 
	
	//.map()
	let veterinary_list: Vec<String> = animals
        .iter()
        .map(|anml| String::from(anml) + " Rescued!")
        .collect();

	//.filter()
	let veterinary_list_filtered: Vec<&String> = animals
        .iter()
        .filter(|anmlf| **anmlf == "Cat".to_owned())
        .collect();

	//.find()
	let veterinary_list_found: Option<&String> = animals
		.iter()
		.find(|anmlf| **anmlf == "Cat".to_owned());

	//.count()
	let veterinary_list_count = animals
		.iter()
		.count();
	
	//.last()
	let veterinary_list_last: Option<&String> = animals
		.iter()
		.last();

	//.min() | .max()
	let number_list_min: Option<&i32> = numbers
		.iter()
		.min();

	let number_list_max: Option<&i32> = numbers
		.iter()
		.max();

	//.take()
	let veterinary_list_take: Vec<&String> = animals
		.iter()
		.take(3)
		.collect();


    println!("To be treated: {:?}",animals);
    println!("Treated: {:?}",veterinary_list);
    println!("Filtered: {:?}",veterinary_list_filtered);
    println!("Found: {:?}",veterinary_list_found);
    println!("Number of elements: {:?}",veterinary_list_count);
    println!("Last item: {:?}",veterinary_list_last);
    println!("Numbers: {:?}",numbers);
    println!("Min: {:?}",number_list_min);
    println!("Max: {:?}",number_list_max);
    println!("Taken: {:?}",veterinary_list_take);
}