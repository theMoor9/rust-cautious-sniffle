## **Rust CheatSheet - Flusso di Controllo**
##### **Table of Contents**
###### [§ Control Expressions](#-Control-Expressions-1) 🚦
- [If - Else](#If---Else)
- [Match](#Match)
##### [§ Advanced Control Expressions](#-Advanced-Control_Expressions-1) 🛂
- [While let](#While-let)
- [If Let Else](#If-Let-Else)
###### [§ Expressions Dynamics](#-Expressions-Dynamics-1) ♻️
- [If - Else](#If---Else-1)
- [Match](#Match-1)
###### [§ Advance Match Dynamics](#-Advance-Match-Dynamics) 🧑‍🤝‍👩
- [Enum](#Enum)
- [Struct](#Struct)
- [Binding Guards](#Binding-Guards)
- [If Guards](#If-Guards)
- [Combined matching](#Combined-matching)
	
___
## **§ Control Expressions**
Espressioni di controllo
	
###  If - Else

- **Descrizione**: Letterale `if` la condizione è vera esegui `{ ... }` `else if` la seconda condizione è vera esegui questo`{ ... }`  `else` esegui questo `{ ... }`
- **Tags**: #if #Controls 
- **Sintassi**: 
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
	
### Match 

- **Descrizione**:  Volendo considerare la sequenza di controllo `if - else` riportata sopra, in caso di possibilità ampie sul controllo della condizione, si creerebbero sequenze controllo `if - else` molto estese. Per cui, per controlli su condizioni singole si predilige la Sintassi: `match`.
- **Tags**: #match #Controls 
- **Esempio**: 

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

##### Match Enumerations

- **Descrizione**:   L'utilizzo del controllo `match` sulle `enum` garantisce un controllo robusto rispetto alle varianti dell'enumerazione, visto che `match` genera un errore se non sono sviluppate tutte le alternative di azione per la  ***variante*** e genera un allerta se anche solo una  ***variante*** non viene utilizzata.
- **Tags**: #match #Enums #Controls 
- **Esempio**: 
	
	 La sua costruzione è effettuata utilizzando il sistema di selezione della  ***variante***
	
```Rust
enum MenuChoice{
	FirstDish(String),
	SecondDish(String),
	Dessert(String),
	Drink(String),
}

// In main
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

// In main
matching_function(choice) // Fucntion call on choice
	```
		
>	Si istanzia `choice` con la variabile `enum` assegnando le varianti come casi di scelta su cui opererà `match` comparando con l'oggetto caso d'analisi `choice`. Si utilizza inoltre `pick` come riferimento di cattura per il contenuto dell'oggetto `choice` ad uso del `println!` come token.
	
##### Match Option

- **Descrizione**: L'utilizzo del controllo `match` sulle `enum Option<T>` garantisce un controllo completo la dove un tipo *Option* presenta la possibilità di valore nullo da gestire
- **Tags**: #match #Option #Controls 
- **Esempio**: 
	
```Rust
enum Activites {
	morning: String,
	evening: String,
	night: Option<String>
}

fn main () {
	
	let today = Activites{
		morning: "Work".to_owned(),
		evening: "Hobby".to_owed(),
		night: None
	}
	
	// We match on an Option possibilities
	match today.night {
		Some("Work") => println!("Get some rest!"),
		Some(activity) => println!("I'm goint to {}", activity),
		None => println!("Good Sleep!")
	}
}
```
	
	
---
## **§ Advanced Control Expressions**
	
**Descrizione**: Seguono espressioni selettive in merito alla gestione dei risultati rispetto i controlli sulle variabili.
	
### If Let Else
	
**Descrizione**: Volendo considerare lo statement di controllo `match` durante i controlli sulle `Option` potrebbe non interessarci effettuare controlli per dare soluzioni su `Option None` si utilizza quindi la serie di comandi If Let Else
**Sintassi**: `if let Some(_) = { ... } else { ... }`
**Tags**: #if 
**Esempio**:
	
```Rust 
let place = Some("Vallombrosa");
match place {
	Some(p) => println!("You are in: {}", p), // Output: You are in: Vallombrosa
	None => println!("I really dont't care where you are")
}

if let Some(p) = place {
	println!("You are in: {}", p); // Output: You are in: Vallombrosa
}

// E' possibile inoltre replicare la struttuara match aggiungendo un else

if let Some(p) = place {
	println!("You are in: {}", p); // Output: You are in: Vallombrosa
} else {
	println!("I really dont't care where you are");
}
```
	
### While Let
	
**Descrizione**: Proprio come `if let` questo comando permette di ciclare rispetto il valore `Some(_)` una variabile `Option`
**Sintassi**: `while let`
**Tags**: #Loops 
**Esempio**:
	
```Rust
let mut place: Option<&str> = Some("Home") ;

while let Some("Home") = place {
	print!("Relax");
	data = None; // Exit loop
}

let animals: [Result<&str, &str>; 5] = [
	Ok("Cat"),
	Ok("Dog"),
	Ok("Shark"),
	Err("No animal in queue"),
	Err("No animal in queue"),
	Err("No animal in queue")
] ;

let mut veterinary_inspection = animals.iter() ;

// Cicla finche è disponibile un valore Some(_)
while let Some(Ok(_)) = veterinary_inspection.next() {
	print!("Visited");
}

```
-  **Output**: `Relax Visited Visited Visited`
	
	
---
## **§ Expressions Dynamics**
	
 E' possibile utilizzare le espressioni in *assegnazione dinamica*. Inoltre è possibile utilizzare le espressioni innestate.
### If - Else
	
- **Uso**: Assegnazione diretta
- Tags: #if  
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
	
### Match 
	
- **Uso**: Assegnazione diretta ed espressioni innestate (Terza variante su `match`).
- **Tags**: #match  
- **Esempio**:
	
```Rust
enum DrinkHad {
	One,
	Two,
	Three,
	Four,
	five(String),
}

fn main () {
	let my_friend_im_writing_drunk_for_real = DrinkHad::Two ;
	let waitress_wantend_me_to_drink: bool = true;
	
	let thought = match my_friend_im_writing_drunk_for_real {
		DrinkHad::One => "I'm a little bit drunk",
		DrinkHad::Two => {
			if waitress_wantend_me_to_drink {
				"I got offered a free vodka topping, \
				She also brought me a free sandwich. \
				I feel dizzy and she likes me!"
			} else {
				"Alright, we're done"
			}
		},
		DrinkHad::Three => "That's sad! Focus!",
		_ => "I would be cooked! I would go home.", // `_` Unnamed alternative
	} ;
	
	println!("{}", thought) ; // Output: I got offered a free vodka...
}
```
	
	
---
## **§ Advance Match Dynamics**

E' possibile rendere i `match` dinamici tramite la personalizzazione delle possibilità con l'uso dei nomi delle variabili (Campi o Varianti) e simboli.
	
### Enums
	
- **Descrizione**: E' possibile dichiarare in maniera dinamica le possibilità non prese in considerazione in modo che il codice sia ricettivo alle alternative.
- **Tags**: #Enums #match  #Advanced 
- **Esempio**:
	
```Rust
enum DrinkHad {
	One,
	Two,
	Three,
	Four,
	five(String),
}
	
fn main () {
	
	let var = 9 ;
	
	let reading = match var {
		1 => "One",
		2 => "Two",
		3 => "Three", 
		other_number => other_numeber, //catches the value of `var`
	} ;
	
	println!("The number read is {}", reading) ; // Output: The number read is 9
	
	let lie_drink_had_today = Drink::Five(String::from("Are you stupid?")) ; 
	
	let second_day_thougth = match lie_drink_had_today {
		DrinkHad::One => "I'm a little bit drunk",
		DrinkHad::Two => "No chance, she was married :\ ",
		DrinkHad::Three => "That's sad! Focus!",
		DrinkHad::Five(good_question) => good_question,
		_ => "I would be cooked! I would go home.",
	} ;
	
	println!("{}", second_day_thougth) ; // Output: Are you stupid?
	
}
```
	
### Struct

- **Descrizione**: Si usa per una ricerca avanzata rispetto i campi, con il supporto della sintassi `..` come strumento di esclusione, e dei nominativi dei campi come carpitori di valore.
- **Tags**: #match #Structs  #Advanced 
- **Esempio**:
	
```Rust
struct Block {
	id: i8,
	field1: i8,
	field2: i8,
	field3: i8,
}

fn search_by_field2 (data: Block) {
	match data {
		// Matchin by field value
		// `..` Are use to not consider other fields
		Block { field2:32, .. } => println!{"Found!"}, 
		//Matching by field catching whatever id value for later use
		Block { field2:22, id, .. } => println!{"Found! id: {}", id},
		_ => println!("Not found!"), 
	}
}

fn main () {
	let first_block = Block {
		id: 10,
		field1: 11,
		field2: 11,
		field3: 11,
	};
	let second_block = Block {
		id: 20,
		field1: 21,
		field2: 22,
		field3: 23,
	};
	let third_block = Block {
		id: 30,
		field1: 31,
		field2: 32,
		field3: 33,
	};
	search_by_hash(third_block) ; 
}
```
- **Output**: Found!
	
### Binding Guards
	
- **Definizione**: E' possibile creare strutture complesse di controllo dentro i `match` statements legando un informazione specifica, tramite la cattura `@`, ad una variabile che potrà essere poi utilizzata. 
- **Sintassi**: `x @ y`
- **Uso**: E' importante per permettere la dichiarazione di una variabile per un utilizzo futuro anziché dichiarare il suo valore in un istanza di controllo.
- **Tags**: #match  #Advanced #Bind
- **Esempio**:
	
```Rust
enum EnumName {
	Variant(i32)
}
let something = EnumName::Variant(2);
match something {
	EnumName::Variant(n @ 2) => println!("Variant {}", n),
	EnumName::Variant(n @ 9 | n @ 3) => println!("Variant 3 or 9"),
	EnumName::Variant(n @ 4..=8) => println!("Range 4 to 8")
}
```
- **Output**: `Variant 2`
	
### If Guards
	
- **Definizione**: E' possibile creare strutture complesse di controllo dentro i `match` statements, tramite gli `if` statements.
- **Uso**: E' importante per permettere controlli rispetto a molteplici variabili in relazione alla variabile che vogliamo controllare.
- **Tags**: #match  #Advanced #if 
- **Esempio**:
	
```Rust
enum EnumName {
	Variant1
	Variant2
	Variant3
}

let number = 9;
let existence = EnumName::Variant3;
match number {
	n if (n == 9 && existence == EnumName::Variant3) => println!("YES"),
	n if existence == EnumName::Variant1 => println!("NO!! {}", n),
}

struct StructName {
	field1: i8,
	field2: i8,
}

let something = StructName{field1:3,field2:9};
match something {
	StructName{field1 , ..} if (number == 9 && field1 == 9) => println!("NO"),
	StructName{.. , field2} if (number == 9 && field2 == 9) => println!("MAYBE"),
}
```
- **Output**: 
```sh
YES
MAYBE
```
	
### Slice Guards
	
- **Definizione**: E' possibile analizzare porzioni di dati con lo *slicing*.
- **Uso**: Si può così eseguire controlli su specifici elementi dello *slice*.
- **Sintassi**: `match vector.as_slice() {}`
- **Tags**: #match  #Advanced #Slices 
- **Esempio**:
	
```Rust
let chart = vec![1,2,3,4,5,6,7,8,9,10]
match chart.as_slice() {
	[fist_plc, .., third_plc,..] => println!("{}st and {}rd",fist_plc,third_plc ),
	[alone] => println("Only one element: {}", alone),
	[] => println!("Empty slice"),
}
```
- **Output**: `1st and 3rd`
	
### Combined matching
	
- **Definizione**: E' possibile combinare l'uso delle nozioni di matching.
- **Tags**: #match  #Advanced 
- **Esempio**:
	
```Rust
let chart = vec![1,2,3,4,5,6]

match chart.as_slice() {
	[podium @ 0..=2 , losers_for_glory @ ..] => {
		// `podium` è sempre uno slice tra 1 e 3
		// `losers_for_glory` è tutto il resto
	},
	[signle] if signle == &5 || signle == &6 => {
		// Solo se chart fosse un unico elemento tra il 5 o il 6
	},
	[..] => println!("I don't care about previously unmatched content"),
	[] => println!("Empty"),
}
```
	
	
---

>E' consigliato usare i `Tags` in relazione a gli altri Cheatsheets per un quadro sull'argomento più esaustivo.
##### Suggested Progression
[Rust CheatSheet - Funzioni](rust-functions-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini
