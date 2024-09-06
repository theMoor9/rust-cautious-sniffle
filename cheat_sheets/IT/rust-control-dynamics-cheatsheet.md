## **Rust CheatSheet - Dinamiche di controllo del codice**
##### **Table of Contents**
###### [§ Control Expressions](#-Control-Expressions-1) 🚦
- [If - Else](#If---Else)
- [Match](#Match)
###### [§ Expressions Dynamics](#-Expressions-Dynamics-1) ♻️
- [If - Else](#If---Else-1)
- [Match](#Match-1)
###### [§ Advance Match Dynamics](#-Advance-Match-Dynamics) 🧑‍🤝‍👩
- [Enum](#Enum)
- [Struct](#Struct)
- Guards
###### [§ Ownership](#-Ownership-1) 🪪
###### [§ Type State](#-Type-State-1)
###### [§ References](#-References-1) 🏷️
- [Trait Objects](#Trait-Objects)
###### [§ Lifetimes](#-Lifetimes-1) ⏱️
- [Funzioni](#Funzioni)
###### [§ Custom Errors](§-Custom-Errors-1) ⚠️
- [Display](#Display)
- [Error Crate](#Error-Crate)
	
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
## **§ Expressions Dynamics**
	
 E' possibile utilizzare le espressioni in *assegnazione dinamica*. Inoltre è possibile utilizzare le espressioni innestate.
### If - Else
	
- **Uso**: Assegnazione diretta
- Tags: #if #Dynamics 
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
- **Tags**: #match #Dynamics 
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
- **Tags**: #Enums #match #Dynamics 
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
- **Tags**: #match #Structs #Dynamics 
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
	
___
## **§ Ownership**
	
La proprietà di una variabile è unica e fine a se stessa fuori da una funzione
    
- **Definizione**: La *Responsabilità* è un attributo atto alla gestione della memoria che garantisce la sicurezza dei dati.
- **Caso d' Uso**: Assicurazione su conflitti di concorrenza e sicurezza. I responsabili della proprietà sono: Funzioni, Closures, Structs, Enums and Scopes.
- **Tags**: #Ownership #Types #Strings #Dynamics
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
	
	
## **§ Type State**

- **Definizione**: E' possibile utilizzare tipi personalizzati per creare degli stati strutture complesse aventi più forme, un po come le varianti di un enumerazione.
- **Uso**: Rappresentare uno stato in campi o variabili. 
- **Tags**: #Types #Dynamics #Custom
- **Esempio**:

```Rust
struct Person<State>{
	name: String,
	condition: State,
}
struct Unemployed{
	from: String,
}

impl Person<Emplyed>{
	fn for_who(name: String, forw: String) -> Self { 
		Self { 
			name, 
			condition: Employed { forw }, 
		} 
	} 
}

fn main(){
	// Crea una persona disoccupata con from When,
	let mut me = Person::for_who("Kenneth".to_owned(), "For you".to_owned());
	
}

```

- ##### ***Approfondimento Avanzato***

```Rust
struct Person<State>{
	name: String,
	condition: State,
}
struct Employed{
	company: String,
}
struct Unemployed{
	from: String,
}

impl Person<Unemployed> { 
	fn from_when(name: String, from: String) -> Self { 
		Self { 
			name, 
			condition: Unemployed { from }, 
		} 
	} 
}
impl Person<Emplyed>{
	fn for_who(name: String, forw: String) -> Self { 
		Self { 
			name, 
			condition: Employed { forw }, 
		} 
	} 
}
impl<State> Person<State> {
	fn change<NewState>(self, state:NewState) -> Person<NewState> {
		Person {
			name: self.name,
			condition: state
		}
	}
}

fn main(){
	// Crea una persona disoccupata con from When,
	let mut me = Person::from_when("Kenneth".to_owned(), "A year".to_owned());
	
	// Modifica me creando una nuova persona da for_who
	let new_year_new_me = me.change(
		Person::for_who("Kenneth".to_owned(), "For you".to_owned())
	);
}

```
## **§ References**
	
- **Definizione**: Attributo che permette di fare *riferimento* ad una variabile *senza* prenderne la *Responsabilità*
- **Sintassi**: `&`
- **Uso**: Prestito di dati *senza* vincoli di *Responsabilità*.
- **Tags**: #References #Types #Strings #Dynamics #Borrowing
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
	
### Trait Object
	
- **Definizione**: Gli _oggetti trait_ (`dyn Trait`) usano il sistema di referenze `&` per operare in modo dinamico. A differenza dei tipi statici, permettono di gestire diversi tipi che implementano lo stesso trait a runtime, con un piccolo costo in termini di velocità e memoria.

>	Quindi un *Trait Object* è l'elemento che ti permette di gestire il codice in runtime.

- **Caso d' Uso**: Le collezioni normalmente contengono tipi statici e definiti al momento della compilazione. Con gli _oggetti trait_, puoi creare collezioni che accettano più tipi diversi, come plugin o estensioni, che possono essere gestiti a runtime. Questo è utile in scenari come videogiochi, sistemi di plugin o quando hai bisogno di modificare dinamicamente il comportamento in base al tipo.
- **Sintassi**: `&dyn Trait`
- **Tags**: #Dynamics #Traits #References #Dynamics
- Esempio:
	
```Rust
// Metodo strutturato
let item = StructItem;
let item_trait_obj: &dyn Trait = &item;

// Metodo diretto
let struct_trait_obj: &dyn Trait = &StructItem;

// Medodo Error-Safe al costo di ingombro e velocità
let boxed_trait_obj: Box<dyn Trait> = Box::new(StructItem);
```
	
	
---
## **§ Lifetimes**
	
- **Definizione**: Secondo il criterio di ownership i tipi che possiedono la proprietà di un certo dato hanno la responsabilità di terminarlo. Per evitare che esso venga terminato si usa la proprietà *lifetime* che ne garantisce l'estensione
- **Uso**: Estendere l'utilità di un elemento nel codice.
- **Tags**: #Dynamics #Lifetimes #Borrowing 
- **Sintassi**: 
	
	```Rust
	struct StructName<'a>{
		field: &'a String 
	}
	
	struct StructName<'static>{
		field: &'static String 
	}
	```
	
	`'a`,`'b`,`'c`...`'z` sono tick e indicano che esiste ownership da un altra parte nel codice
	`field: &'a String` si traduce in: 
		il campo `field` aspetta un prestito `&` con ownership esterna `'a` da proprietario `String`
    `'static` indica che la variabile ha durata uguale al programma stesso.

- **Esempio**:
    
    ```Rust
    
	struct StructName < 'a,'b,'c,'static >{
	    field1: &'a EnumName, // Durata di EnumName
	    field2: &'b String, // Durata di string
	    field3: &'c Struct2Name, // Durata di Struct2Name
	    field4: &'static Enum2Name, // Durata dello script
	    field5: u8 //Durata di StructName
    }
	
	// Chiamata 
	let new_variable = StructName{
		field1: &EnumName::Version,
		...
	}
    ```
	
> Una volta concluso l' uso di StructName le altre variabili rimarranno disponibili visto che sono riferimenti il field5 no. 
> 
> **Le strutture che prendono in prestito inoltre devono essere sempre create dopo chi presta la variabile e il suo contenuto e distrutte prima che il proprietario venga terminato**
	
### Funzioni
	
- **Descrizione**: Se si vuole ritornare un campo di una struttura come referenza tramite funzione occorre implementare il *lifetime*. Non è una pratica comune ma è possibile farlo nel caso sia necessario.
- **Tags**: #Functions #Lifetimes #Borrowing 
- **Esempio**:
	
```Rust
fn function_name<'a>(arg: &'a DataType) -> &'a DataType {/*Corpo di mille balene*/}
```
	
	
---
## **§ Custom Errors**

- **Definizione**: Gli errori personalizzati sono costruiti attraverso l'uso delle enumerazioni, devono implementare con `derive` il trait di `Debug`, `Error` e `Display`.
- **Uso**: Garantire una comprensione migliore delle eventuali problematiche che potrebbero verificarsi nel codice raggruppandoli per tipo. Preferibile rispetto ai messaggi string, con lo statement `match` i custom 
- **Tags**: #Dynamics #Error #Traits #Display #Enums 
- **Esempio**: 
	
```Rust
#[derive(Debug)]
enum ErrorGenotype {
	NetworkError,
	NotAthorized(i32),
	WrongInput,
}
use::std::Error; // Dalla libreria standard importiamo il trait Error
impl Error for ErrorGenotype {} //Vuoto
// L'implementazione è opzionale visto che ErrorGenotype lo abbiamo già

fn function() -> Result<(),ErrorGenotype> {
	//body
	Err(ErrorGenotype::WrongInput)
}
```
	
### Display
	
- **Definizione**: L'Implementazione opzionale di `Display` tramite la libreria `std`. L'intera sintassi è di default, l'unica parte dell' implementazione da modificare è lo sviluppo del `match` interno. 
- **Uso**: L' implementazione `Display` serve per riportare i messaggi di errore personalizzati a a livello utente. 
- **Tags**:
- **Esempio**:
	
```Rust
use std::fmt; // Format
impl fmt::Display for ErrorGenotype {
	fn fmt(&self, f: &mut fmt::Formatter -> fmt::Result {
		match self {
			Self::NetworkError => write!(f, "Network error check connection"),
			Self::NotAuthorized(code) => {
				match code {
					1 => write!(f, "You don't have the necessary permissions"),
					2 => write!(f, "You need to sign in first"),
				}
			},
			Self::WrongInput => write!(f, "Type better!"),
		}
	}
}
```
	
### Error Crate
	
- **Definizione**: Questo crate permette una versione della gestione degli errori meno macchinosa generando i messaggi in maniera diretta tramite le annotazioni al posto della creazione del `Display` trait.
- **Tags**: #Crates #Error #Enums 
- **Uso**:
	
```Rust
// In cargo.toml
[Dependencies]
thiserror = "1.0"
```
	
```sh
// In terminal per utima versione garantita
cargo install thiserror
```
	
- **Esempio**:
	
```Rust
use thiserror::Error

#[derive(Debug,Error)]
enum ErrorGenotype {
	#[error("Network error check connection")]
	NetworkError,
	#[error("Not authorized: {0}")] 
	NotAthorized(i32), // {0} indica la posizione della tupla di NotAuthorized
	#[error("Type better!")]
	WrongInput,
}
```
	
>In questo modo abbiamo un sistema veloce ma meno personalizzato per la gestione degli errori.
	
###### *Approfondimento Avanzato*
- **Descrizione**:  E' possibile innestare le enumerazioni per ottenere versioni specifiche dell'errore.
- **Uso**: Permettere la propagazione con `?` .
- **Tags**: #Crates #Error  #Enums #Advanced 
- **Esempio**:
	

```Rust
use thiserror::Error

#[derive(Debug,Error)]
enum NetworkError {
	#[error("Connection timed out")]
	TimeOut
	#[error("Unreachable")]
	Unreachable
}


#[derive(Debug,Error)]
enum ErrorGenotype {
	#[error("Network error check connection")]
	Connection(#[from]NetworkError),
	#[error("Not authorized: {0}")] 
	NotAthorized(i32), // {0} indica la posizione della tupla di NotAuthorized
	#[error("Type better!")]
	WrongInput,
}
let error = ErrorGenotype::Connection(NetworkError::TimeOut);
println!("{}", error); // Errore di connection
println!("{}", error?); // Errore di propagato di NetworkError::TimeOut
```
	
- **Output**: 
	
```sh 
Network error check connection
Connection timed out
```
	
---
##### Suggested Progression
[Rust CheatSheet - Funzioni](rust-functions-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini
