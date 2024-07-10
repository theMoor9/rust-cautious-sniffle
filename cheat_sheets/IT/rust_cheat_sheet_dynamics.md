# Rust Cheat Sheet - Dinamiche del codice
Dinamiche di funzionamento e proprietà del linguaggio
	
___
## **Control Expressions**
Espressioni di controllo
	
- ###  If - Else
	Sintassi: 
	```Rust
	if condition && condition2 {
		// Esecuzione del codice
	} else if condition || condition2 {
		// Esecuzione alternativa condizionale del codice
	} else {
		// Esecuzione alternativa del codice
	}
	```
	
	> Spesso è possibile abbreviare i controlli
	
- ### Match 
	
	Volendo considerare la sequenza di controllo `if - else` riportata sopra, in caso di possibilità ampie sul controllo della condizione, si creerebbero sequenze controllo `if - else` molto estese. Per cui, per controlli su condizioni singole si predilige la Sintassi: `match`.
	
	Logica  ***inefficiente*** `if - else`
	```Rust
	let mut condition = "White"
	
	if condizion == "Gold"{
		println!("Il colore è {}", condition) ;
	} else if condition == "Purple"{
		// Esecuzione alternativa condizionale del codice
	} else if condition == "Navy"{
		// Esecuzione alternativa condizionale del codice
	} else if condition == "Green"{
		// Esecuzione alternativa condizionale del codice
	} else {
		println!("Il colore è non in lista") ;
	}
	// Restituisce a monitor "Il colore è non in lista"
	```
	
	Logica ***efficiente*** di `match`
	```Rust
	let mut condition = "White"
	
	// Si attesta la variabile da controllare a `match`
	match  condition{
		// I possibili valori vengono listati e separati da una virgola
		"Gold" => println!("Il colore è {}", condition),
		"Purple" => println!("Il colore è {}", condition),
		"Navy" => println!("Il colore è {}", condition),
		"Green" => println!("Il colore è {}", condition),
		_ => println!("Il colore è non in lista"),
	} ;
	// Restituisce a monitor "Il colore è non in lista"
	```
	
	>L'espressione `match` esegue il computo che è posto dopo `=>` inerente al valore di `condition` al momento del controllo
	>
	>Underscore `_` è utilizzato come si utilizzerebbe un `else` non condizionale, per coprire eventuali opzioni extra
	
	- ##### Enumerations
	
	 L'utilizzo del controllo `match` sulle `enum` garantisce un controllo robusto rispetto alle varianti dell'enumerazione, visto che `match` genera un errore se non sono sviluppate tutte le alternative di azione per la  ***variante*** e genera un allerta se anche solo una  ***variante*** non viene utilizzata.
	
	 La sua costruzione è effettuata utilizzando il sistema di selezione della  ***variante***
	
	
	```Rust
	enum MenuChoice{
		FirstDish(String),
		SecondDish(String),
		Dessert(String),
		Drink(String),
	}
	
	let choice = MenuChoice::SecondDish(String::from("Fiorentina steak")) ;
	
	match choice {
		MenuChoice::FirstDish(pick) => println!("He ordered {}", Pick),
		MenuChoice::SecondDish(pick) => println!("He ordered {}", Pick),
		MenuChoice::Dessert(pick) => println!("He ordered {}", Pick),
		MenuChoice::Drink(pick) => println!("He ordered {}", Pick),
	} ;

	// Othewise
	
	fn matching_function (preference: MenuChoice) {
		match preference{
			MenuChoice::FirstDish(pick) => println!("He ordered {}", pick),
			MenuChoice::SecondDish(pick) => println!("He ordered {}", pick),
			MenuChoice::Dessert(pick) => println!("He ordered {}", pick),
			MenuChoice::Drink(pick) => println!("He ordered {}", pick),
		}
	}
	
	matching_function(choice) // Fucntion call on choice
	```
	
  >Si istanzia `choice` con la variabile `enum` assegnando le varianti come casi di scelta su cui opererà `match` comparando con l'oggetto caso d'analisi `choice`. Si utilizza inoltre `pick` come riferimento di cattura per il contenuto dell'oggetto `choice` ad uso del `println!` come token.
	
	
---
## **Expressions**
	
 E' possibile utilizzare le espressioni in *assegnazione dinamica*. Inoltre è possibile utilizzare le espressioni innestate.
##### If - Else
	
- **Caso d'Uso**: Assegnazione diretta
- **Esempio**:
	
```Rust
let control_variable: u32 = 10;
let mut variable: bool = if control_variable > 9 {
	true
} else {
	false
}
```
	
E' possibile utilizzare le espressioni per costrutti diretti attraverso l'*assegnazione dinamica*.
Siccome l'espressione `if`  restituisce un valore booleano e il controllo stesso è un'entità booleana, si effettua quanto segue.
	
```Rust 
let control_variable: u32 = 10 ;
let mut variable: bool = control_variable > 9 ;
```
	
##### Match 
	
- **Caso d'Uso**: Assegnazione diretta ed espressioni innestate (Terza variante su `match`).
- **Esempio**:
	
```Rust
	enum DrinkHad {
		One,
		Two,
		Three,
		Four,
	}
	
	let my_friend_im_writing_drunk_for_real = DrinkHad::Two ;
	let waitress_wantend_me_to_drink: bool = true;
	
	let thought = match my_friend_im_writing_drunk_for_real {
		DrinkHad::One => "I'm a little bit drunk",
		DrinkHad::Two => {
			if waitress_wantend_me_to_drink {
				"I got offered a free vodka topping, \
				She also brought me a free sandwich. \
				I feel dizzy but she likes me!"
			} else {
				"Alright, we're done"
			}
		},
		DrinkHad::Three => "That's sad! Focus!",
		_ => "I'm cooked! Go home!",
	} ;
	
	println!("{}", thought) ; // Output: I got offered a free vodka...
```
	
	
___
## **Ownership and References**
	
La proprietà di una variabile è unica e fine a se stessa fuori da una funzione
	
##### Ownership:
    
- **Definizione**: La *Responsabilità* è un attributo atto alla gestione della memoria che garantisce la sicurezza dei dati.
- **Caso d' Uso**: Assicurazione su conflitti di concorrenza e sicurezza.
- Esempio:
	
	```Rust
	let s = String::from("hello"); 
	let s1 = s; // s è spostato in s1, s non è più disponibile
	// println!("{}", s); // Questo causerebbe errore


	fn main () {
		let s2 = 10;
	}
	// println!("{}", s2) // Questo causerebbe errore
	```
	
	
	>Se la variabile si trova all interno di una funzione, essa prende la *Responsabilità* fino alla fine del suo blocco. Alla fine di essa il dato variabile viene eliminato.
		
	
```Rust
struct Customer {
	name: &str,
	age: u8,
	email: &str,
}

fn register_customer (i : &Customer) {
	println!("{} succesfully registered!", i.name);
}
fn display_age (i: &Customer) {
	println!("name: {} age: {}", i.name i.age)
}

fn main () {
	let me = Customer{
		name: "Kenneth"
		age: 27
		email: "kennethhereiam@mydomain.com"
	}
	register_customer(&me)
}
```
	
##### References:
	
- **Definizione**: Attributo che permette di fare *riferimento* ad una variabile *senza* prenderne la *Responsabilità*
- **Sintassi**: `&`
- **Caso d'Uso**: Prestito di dati *senza* vincoli di *Responsabilità*.
- **Esempio**:
	
```Rust
// Si aggiunge la refernza `&` nell'intestazione
fn calculate_length(given_name: &String) -> usize {
	given_name.len() 
}

let name = String::from("Kenneth"); 

// Si aggiunge la referenza `&` quando si chiama la funzione
let len = calculate_length(&name); 

println!("The length of the name '{}' is {}", name, len);  
/*
Questo è possibile per la presenza della referenza `&`
Considerato che `calculate_length` non elimina la 
variabile che elabora alla fine del suo contesto
*/
```
	
- **Output**: `The length of the name Kenneth is 7
	
	>Non è possibile utilizzare i _Riferimenti_ `&` nei tipi complessi come `struct` ed `enum` perché questi sono obbligati alla pulizia della memoria occupata alla fine del loro scopo. Poiché i _Riferimenti_ non sono di loro proprietà, il compilatore genera un errore per garantire l'integrità del codice.
	>
	>Per ovviare a questo problema, sono necessarie le annotazioni di durata ***lifetimes*** oppure, in alternativa, si può usare la `struct` di default di Rust `String`.
	
---
### **Lifetimes** da finire
	
- Ensure that references are valid as long as they are used.
- **Caso d'Uso**: Managing the scope of references.
- **Esempio**:
    
    ```
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
		    x     
        } else {
		    y    
	    }
	}  
	
	let string1 = String::from("long string is long"); 
	let string2 = "xyz"; 
	let result = longest(string1.as_str(), string2); 
	println!("The longest string is {}", result);
    ```
	
- **Output**: `The longest string is long string is long`
	
	
---
	
**Author:** Kenneth Boldrini
