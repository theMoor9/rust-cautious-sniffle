use std::collections::HashMap;

#[derive(Debug)]
struct Guest {
	name: String,
	surname: String,
	telephone: u32,
	email: String,
	confirm: bool
}
	


fn add_confirms (chr: &mut HashMap<String,Guest>, hrl: Vec<Guest>) {
	// Componente per l' ID
	let mut n = 0 ; 

	for g in hrl {
        // Se è confermato inserisce
		if g.confirm {
            // Converte n in stringa per creare l' ID
			let nstring = n.to_string();
			let id: String = "ID".to_owned() + &nstring + " ";
            // Iserisce nell HashMap ID e Guest
			chr.insert(id,g) ;
            // Aumenta il valore ID
			n = n + 1
		}
	}
}	

fn see_confirms (chr: &HashMap<String,Guest>) {
    for (id,g) in chr.iter() {
        println!("{}{:?}",id,g);
    }
}

fn main() {

    // Lista delle prenotazioni non confermate
    let mut hotel_reservation_list: Vec<Guest> = vec![
        Guest{
            name: String::from("Kenneth"),
            surname: String::from("Boldrini"),
            telephone: 345345345,
            email: String::from("kenneth@email.com"),
            confirm: true
        },
        Guest{
            name: String::from("Linda"),
            surname: String::from("Francescucci"),
            telephone: 346346346,
            email: String::from("linda@email.com"),
            confirm: true
        },
        Guest{
            name: String::from("Niccolò"),
            surname: String::from("Pierazzi"),
            telephone: 347347347,
            email: String::from("niccolo@email.com"),
            confirm: false
        },
    ];

    // Lista delle prenotazioni confermate
    let mut confermed_hotel_reservations:  HashMap<String,Guest> = HashMap::new() ;
    // Inserimento conferme
    add_confirms(&mut confermed_hotel_reservations,hotel_reservation_list);
	see_confirms(&confermed_hotel_reservations);
}