# **Rust CheatSheet - Tipi**
##### **Table of Contents**
###### [§ Tipi Scalari](#-Tipi-Scalari-1)
- [Numeri Interi](#Numeri-Interi)
- [Numeri Floating-Point](#Numeri-Floating-Point)
- [Booleani](#Booleani)
- [Caratteri](#Caratteri)
- [Unit](#Unit)
###### [§ Tipi Composti](#-Tipi-Composti-1)
- [Tuples](#Tuples)
- [Arrays](#Arrays)
###### [§ Tipi Complessi](#-Tipi-Complessi-1)
- [Struct](#Struct)
- [Enumerations](#Enumerations)
###### [§ Tipi Aggiuntivi](#-Tipi-Aggiuntivi-1)
- [Slices](#Slices)
- [String Slices](#String-Slices)
- [Option](#Option)
###### [§ Annotazioni Esplicite del Tipo](#-Annotazioni-Esplicite-del-Tipo)
- [Specifiche](#Specifiche)
- [Generiche](#Generiche)
###### [§ Traits](#-Traits-1)
- [Implementazione](#Implementazione)
- [Default](#Default)
- [Funzioni Generiche](#Funzioni Generiche)
- [Struct Generiche](#Struct Generiche)
	
---
## **§ Tipi Scalari** 
	
I tipi scalari in Rust rappresentano i valori più semplici, quelli che non possono essere suddivisi ulteriormente.
	
### Numeri Interi
    
- **Interi Polari**: (`i` : Integers) Possono assumere forma negativa e positiva.
	
	- **Tipi**: ```i8```, ```i16```, ```i32```, ```i64```, ```i128```, ```isize```
	- **Uso**: Immagazzinare Temperatura, e variabili che possono assumere valori negativi.
	- **Tags**: #Integers 
	- **Esempio**:
	
		```Rust
		let age: i32 = 30; 
		let temperature: i16 = -5;
		```           

- **Interi Non Polari**: (`u`: Unsigned) Sono solo numeri positivi.
	
	- **Tipi**: ```u8```, ```u16```, ```u32```, ```u64```, ```u128```, ```usize```
	- **Uso**: Età, Indicizzazione array, etc.
	- **Tags**: #Integers 
	- **Esempio**:
	
		```Rust
		let item_count: u32 = 50; 
		let index: usize = 3;
		```
            
### Numeri Floating-Point
    
- **Definizione**: Usato per i numeri decimali.
- **Tipi**: ```f32``` (Precisione singola), ```f64``` (Precisione doppia)
- **Uso**: Calcoli scientifici e di precisione.
- **Tags**: #Float 
- **Esempio**:
	
	```Rust
	let pi: f32 = 3.14; 
	let e: f64 = 2.718281828459045;
	```
        
### Booleani
    
- **Definizione**: Rappresentano vero o falso.
- **Tipo**: ```bool```
- **Uso**: Controlli condizionale e avvisi flag.
- **Tags**: #Bool 
- **Esempio**:
	
	```Rust
	let is_active: bool = true; 
	let has_error: bool = false;
	```
	
#### Caratteri:
    
- **Definizione**: Rappresenta un codice univoco, racchiuso tra apici.
- **Tipo**: ```char```
- **Uso**: Immagazzinare caratteri e simboli singoli.
- **Tags**: #Char 
- **Esempio**:
	
	```Rust
	let letter: char = 'A'; 
	let emoji: char = '😊';
	```
	
#### Unit
	
- **Definizione**: E' una tupla senza campi.
- **Tipo**: ```()```
- **Uso**: Specificare valore assente la dove è richiesto un valore obbligatoriamente ad esempio su un ritorno.
- **Tags**: #Unit #Result
- **Esempio**:
	
```Rust
 fn func () -> Result<(),i64>{
	 ...
 }
```
	
	
---
## **§ Tipi composti**
	
I tipi composti sono quelli che combinano insieme altri tipi in una struttura semplice. Sono tipi "contenitore" che non hanno necessariamente comportamento o logica associata, ma sono semplicemente un modo di organizzare più valori insieme.
	 
### Tuples
	
- **Definizione**: Può raggruppare molteplici valori con differenti tipi
- **Uso**: Ritornare valori multipli da una funzione.
- **Tags**: #Tuples 
- **Esempio**:
	
```Rust
// Unpacking della tupla in variabili definite
let (first, second, third) = (1,2,3) ; 

// Packing delle variabili in una tupla
let numbers = (first, second, third) ; 

// Accesso tramite posizione dell'elemento
println!("The First Number Is {}", numbers.0) ; // Selezione posizionale .1
// Visualizza a monitor "The First Number Is 1"

println!("The Second Number Is {}?", numbers.1) ; // Selezione posizionale .2
// Visualizza a monitor "The Second Number Is 2?"

println!("The Third Number Is {}!", numbers.2) ; // Selezione posizionale .3
// Visualizza a monitor "The Third Number Is 3!"
```
	
> Si suggerisce di usare le `struct` quando si ha più di due o tre ***campi***
        
	
### Arrays
    
- **Definizione**: Una collezione di elementi  dalla lunghezza e il tipo prefissati.
- **Uso**: Immagazzinare dei dati fissi come i giorni della settimana.
- **Tags**: #Arrays 
- **Esempio**:

```Rust
let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
let first_day = days[0]; 
println!("First day: {}", first_day);
```
- **Output**: `First day: Mon`
	
	
---
## **§ Tipi Complessi**
	
I tipi complessi, sono quelli che combinano dati e logica. Possono contenere variabili di tipi diversi, ma hanno anche metodi associati che definiscono comportamenti e interazioni tra i dati.
	
### Struct
	
- **Definizione**: E' un tipo complesso che ha scopo di definire un oggetto avente le "proprietà", chiamate ***campi***, della struttura. I campi si definiscono con loro tipo.
- **Tipo**: `struct`
- **Uso**: Definizione di oggetti con proprietà fisiche e metafisiche
- **Tags**: #Structs 
- **Esempio**:
	
```Rust
struct ShippingBox{
	width: u32, 
	height: u32,
	depth: u32,
}

/* 
Si u32 invece di i32 perchè chiaramente nessuna scatola 
può essere di misura negativa
*/

let my_box = ShippingBox {
	width: 9,
	height: 6,
	depth: 3,
} ; 
```
	
Si chiama in causa un ***campo*** della struct utilizzando il punto `.`
	
```Rust
let my_box_height = my_box.height ;
let my_box_depth = my_box.depth ;
let my_box_width = my_box.width ;

let my_box_volume = my_box.width * my_box.depth * my_box.height ;
```
	
>Le proprietà di solidità di Rust impongono che alla creazione `let` di una `struct` si debba specificare tutti i ***campi*** di essa per evitare errori
	
- ##### Struct String 
	
	- **Definizione**: E' una collezione mutabile e dinamicamente allocata di caratteri UTF-8.
	- **Tipo**: `String`
	- **Uso**: Usa `String` per stringhe mutabili e possedute, e `&str` per riferimenti immutabili a stringhe esistenti.
	- **Tags**: #Structs #Strings 
	- **Esempio**:
		
	```Rust
	// Init 
	let mut s = String::from("Hello"); 
	
	// Modify 
	s.push_str(", world!"); 
	s.push('!'); 
	
	// concatenate 
	let s2 = String::from(", How are you?"); 
	let s3 = s + &s2; // s viene spostato qui e non può più essere usato 
	
	// Iteration on string's chars
	for c in s3.chars() { 
		println!("{}", c); 
		} 
		
	// Substring access 
	let hello = &s3[0..5]; let world = &s3[7..]; 
	println!("Substring 1: {}", hello);
	// Output: Hello 
	println!("Substring 2: {}", world); 
	// Output: world! How are you? 
	```
	
	- **String Slice** `&str`:
	
	```Rust
	// Conversione tra String e &str 
	let slice: &str = &s3; 
	let s4 = slice.to_string(); 
	println!("Original String: {}", s3); 
	println!("Converted String: {}", s4); 
	```
	
	- **Other useful functions**:
	
	```Rust
	let len = s4.len(); // Length
	let is_empty = s4.is_empty(); // Clear right?
	println!("Length: {}", len); 
	// Output: 22 
	println!("Is Empty: {}", is_empty); 
	// Output: false 
	let mut s5 = String::from("Temporary"); 
	s5.clear(); 
	println!("Cleared String: {}", s5); 
	// Output: (empty string)
	```
	
>	Con lo `Struct String`, il valore nella memoria, quando viene modificato usa lo stesso spazio piuttosto creare un altro valore nella memoria. Agendo sul valore della variabile come una proprietà dell'oggetto "classe"

- ##### Struct Vec

	- **Definizione**: Una collezione di dati dal tipo univoco dalla dimensione variabile
	- **Sintassi**: `Vec<>`
	- **Uso**: Collezionare dati dello stesso tipo variabile come la lista di clienti
	- **Tags**: #Structs #Vectors 
	- **Esempio**:
		
	```Rust
	let slice: Vec<i32> = Vec::new() ;
	println!("{:?}", slice); // Output: []
	
	let ten_positions: Vec<&str> = Vec::with_capacity(10) ;
	println!("Capacity: {}", ten_positions.capacity()) ; // Output: Capacity: 10
	
	let vector: Vec<i32> = Vec::from(slice) ; // Output: [1, 2, 3, 4, 5]
	
	vector.push(1); 
	vector.push(99);
	println!("{:?}", v); // Output: [1, 2, 3, 4, 5, 1, 99]
	
	let last = vector.pop()
	println!("Last position: {}", last); // Output: [99]
	
	let second = vector.remove(1);
	println!("{:?}", second); // Output: 2 
	println!("{:?}", vector); // Output: [1, 3, 4, 5, 1]
	```
	
### Enumerations 
	
- **Definizione**: E' un tipo complesso che può assumere uno tra diversi valori definiti, dove ogni _**variante**_ può contenere dati di un tipo specifico come `String`, `char`, `int`, `float`, ecc.
- **Tipo**: `enum`
- **Uso**: Quando si necessita che un'entità possa avere più versioni di se stessa.
- **Tags**: #Enums 
- **Esempio**:
	
```Rust
enum EnumName {
	Version1(u8),
	Version2(i8),
	Version3{
		key1:i8, 
		key2:&str, 
		key3:OtherEnum, 
	},
	Version4,
}

fn main () {
	let enum_variable1 = EnumName::Version1(9) ;
	let enum_variable2 = EnumName::Version2(-9) ;
	let enum_variable3 = EnumName::Version3{
		key1: 96,
		key2: "Questa è roba puzzle!",
		key3: OtherEnum::SomeVersion.. ,
	} ;
}
```
	
##### *Approfondimento Avanzato*
	
- **Descrizione**:  Enumerazioni contenenti versioni con valori multipli e selezione appropriata.
- **Tags**: #Enums #Advanced
- **Esempio**:
	
```Rust
enum Exceptions {
	NoLactose(bool),
	GlutenFree(bool),
	Normal(bool),
	SizeBig(bool),
}

enum Menu { 
	Pizza(String), // Tipo di pizza 
	Pasta { name: String, recipe: Exceptions }, // Tipo di pasta 
	Beverage { name: String, volume: f32 }, // Nome e volume in litri 
	Dessert, // Dolce senza dati associati 
}

/* 
Si seleziona la variante dell'enumerazione con `::`
Ne si dichiara il contenuto
*/
fn main () {
	let first_pick = Menu::Pizza(String::from("Prataiola")) ;
	let second_pick = Menu::Pasta { 
		name: String::from("Ragù"), 
		recipe: Exceptions::NoLactose(true),
	} ;
	let second_pick_beverage = Menu::beverage{ 
		name: String::from("Beer"), 
		volume: 0.5
	} ;
}
```
	
>	Da notare la possibilità di creare tipi piramidali complessi con varianti di tipo `enum`
	
- **Argomenti**: Creare un enumerazione con <> equivale a creare una sezione per gli  >argomenti argomenti  posizionabili al suo interno come valori standard delle varianti.
	
```Rust
enum PlanetsIT <M, D, R,> {
	Mercurio(M, D, R),
	Venere(M, D, R),
	Terra(M, D, R),
	Marte(M, D, R),
	Saturno(M, D, R),
	Giove(M, D, R),
	Urano(M, D, R),
	Nettuno(M, D, R),
	BigRockNotAPLanetAnymorePlutone(M, D, R),
}

fn main () {
	let diameter = String::from("Less than current");
	let mass = String::from("Less than current");
	let around_sun_revolutions = String::from("Somewhat similar");
	let our_next_planed: PlanetIT = PlanetIT::Marte(
		mass,
		diameter,
		around_sun_revolutions
	);
}
```
	
### Hashmaps
	
- **Definizione**: E' un tipo complesso riconducibile come analogia ai dizionari, set di dati di tipi disparati con chiavi associate registrati in maniera casuale, quindi le chiavi diventano essenziali per la gestione.
- **Libreria**: `use std::collections::HashMap;`
- **Sintassi**: `let mut dictionary = HashMap::new();`
- **Uso**: Metodo funzionale e veloce di gestire i dati . Utile per il recupero dati intuitivo via chiave.
- **Tags**: #Hashmaps #Option 
- **Esempio**:
	
```Rust
use std::collections::HashMap;

let mut dictionary = HashMap::new() ; // Init

let pages1 = 90 ;
let pages2 = "A thousand" ; // I valori possono essere disparati

// Si definisce chiave e valore per inserire i dati
dictionary.insert("Key Book1", pages2) ; // Simile al .push per i vettori
dictionary.insert("Key Book2", pages1) ;

// Si indica la chiave per rimuovere il blocco col valore associato
dictionary.remove("Key Book1") ; // Simile al .pop per i vettori

// `.get` restituisce un Option value gestibile con `match`
match dictionary.get("Key Book2") {
	some(pagesamount) => println("{}",pagesamount),
	None => println("Not found")
}

// ITERARE un HashMap

let mut hotel_rooms = HashMap::new();

hotel_rooms.insert("Client ID 1","Room Number1");
hotel_rooms.insert("Client ID 2","Room Number2");
hotel_rooms.insert("Client ID 3","Room Number3");

// .iter is used to go through considering the couples KEY/VALUE
for (client, room) in hotel_rooms.iter() {
	println!("Client ID: {} and its room number: {}", client, room);
}

// Go through KEYs
for client in hotle_room.key(){
	println!("Client ID: {}", client);
}
// Go through VALUEs
for rooms in hotle_room.values(){
	println!("Client room number: {}", room);
}
```
	
##### *Approfondimento Avanzato*
	
- **Descrizione**:  Costruire una struttura di dati basata su chiavi utilizzando una libreria standard dalla API Doc.
- **Tags**: #Hashmaps #Advanced
- **Esempio**:
	
```Rust
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
    let mut confermed_hotel_reservations:  HashMap<String,Guest> = HashMap::new() ;
    // Inserimento conferme
    add_confirms(&mut confermed_hotel_reservations,hotel_reservation_list);
    see_confirms(&confermed_hotel_reservations);
}
```
	
	
---
## **§ Tipi Aggiuntivi**
	
### Slices
	
- **Definizione**: Visualizzazione dinamica di un *array*.
- **Uso**: Accedere ad una porzione di un array .
- **Tags**: #Slices #Vectors #Arrays 
- **Esempio**:
	
```Rust
let array = [1, 2, 3, 4, 5]; 
let slice: &[i32] = &array[1..3]; 
println!("Slice: {:?}", slice);
```
- **Output**: `Slice: [2, 3]`
	
### String Slice  
	
- **Definizione**: (```&str```) Riferimento immutabile ad una stringa
- **Uso**: Gestire i dati di testo in maniera temporanea o  per la creazione di una frase definitiva convertendola in struct `String`.
- **Tags**: #Slices #Strings  
- **Esempio**:
	
	```Rust
	let greeting: &str = "Hello"; 
	let mut message = String::from("Hello"); 
	message.push_str(", world!"); 
	println!("{}", message);
	```
- **Output**: `Hello, world!`
	
### Option
	
- **Definizione**: Il tipo ***Option*** è un `enumerazione` predefinita di Rust
	
	```Rust
	enum Option<T>{
		Some(T),
		None
	}
	```
	
- **Uso**: Dare la possibilità ad una variabile di assumere variante simboleggiante valore nullo, in attesa di un assegnazione.
- **Sintassi**: `Option<Type>`
- **Tags**: #Types #Option #Enums 
- **Esempio**:
	
```Rust
struct Shipping {
	address: String,
	address_number: civic_number,
	city: String,
	email: String,
	// Option type definition
	telephone: Option<i8>, 
	gift_option: Option<bool>
}

let my_shipping = Shipping {
	address: String::from("Via Sab Niccolò"),
	address_number: 16,
	city: String::from("Firenze"),
	email: String::from("jhondoe@protonmail.com"),
	// Instantiation of the Option variant and its possible type
	telephone: None,
	gift_option: Some(True) 
}
```
	
##### *Approfondimento Avanzato*
	
- **Descrizione**:  Un sistema di ricerca per valore che risulta in un None in caso di fallimento.
- **Tags**: #Option #Advanced
- **Esempio**:
	
```Rust
struct PersonID {
	name: String,
	age: u8,
	email: Option<String>
}

fn find_age (people: Vec<PersonID>, name: &String) -> Option<u8> {
	for person in people {
		if person.name == name {
			return person.age
		}
	}
	None
}

fn main () {
	let travelers = vec![
		PersonID { name: "Kenneth".to_owned(), age:27, email: None},
		PersonID { name: "Linda".to_owned(), age:19, email: None },
		PersonID { name: "Elisa".to_owned(), age:21, email: None },
		PersonID { name: "Lorenzo".to_owned(), age:27, email: None },
	];

	let me = "Kenneth"
	let kenneth_age = find_age(travellers, &me);
}

```
	
### Result
	
- **Definizione**: Il tipo ***Result*** è un `enumerazione` predefinita di Rust
	
```Rust
enum Result <T,E> {
	Ok(T),
	Err(E)
}
```
	
- **Uso**: Dare la possibilità ad una variabile di assumere una variante con valore extra. Utile per la gestione degli errori, accompagnato da un `match` per gestire l' alternativa. La funzionalità è analoga al blocco `try/catch-except`  
- **Sintassi**: `Result<Type, Type>`
- **Tags**: #Types #Result #Enums 
- **Esempio**:
	
```Rust
// Nella firma della funzione si dichiara Result e i tipi delle varianti
fn distance_goal_check (mt: f64, minutes: u8) -> Result<u8, String>{
	if mt > 1000 {
		// Ritorna una variante Ok `enum Result`
		Ok(minutes)
	} else {
		// Ritorna una variante Err `enum Result`
		Err(String::from("Not yet!"))  
	}
}

fn main () {

	let mut distance = 999;
	let mut time = 60;
	
	// Diventa una variabile variante di enum Restult
	let control = distance_goal_check(distance,time) ;

	match control {
		Ok(t) => println!("Your time is: {}", t),
		Err(e) => println!("{}",e),
	}
}
```
	
##### *Approfondimento Avanzato*
	
- **Descrizione**:  Si può evitare controlli match sul tipo di *Result* ottenuto.
- **Sintassi**: `?`
- **Uso**: Si usa per dare la possibilità ad una chiamata di funzione che restituisce un *Result* di autogestirsi dentro ad una funzione che ritorna un *Result* dando il risultato se esiste o l'errore alla funzione madre se si verifica posizionando `?` subito dopo la funzione figlia.
- **Tags**: #Result #Advanced
- **Esempio**:
	
```Rust
#[derive(Debug)]
enum MenuPath {
    Start,
    Options,
    Exit
}

// Funzione di definizione dei valori dei tipi di Result
fn choice (input: &char) -> Result<MenuPath, String> {
    match input {
        's' => Ok(MenuPath::Start),
        'o' => Ok(MenuPath::Options),
        'e' => Ok(MenuPath::Exit),
        _ => Err("Wrong input!".to_owned()),
    }
}

// print_path accetta solo tipi enum MenuPath 
fn print_path (c: &MenuPath) {
    println!("{:?}",c);
}

// Il return Result è atto a catturare gli errori omettendo T con ()
fn determine_path ( input: char) -> Result<(),String> {
    /*
	Determiniamo checked_choice come tipo enum MenuPath
	L' operatore `?` ritornerà un Err come Result per 
	determine_path interrompendo la funzione
	*/
	let checked_choice: MenuPath = choice(&input)?;
	// E' assicurato che in questo caso print_path non riceva un Err
	print_path( &checked_choice );
	Ok(())
}

fn main () {
	// Se è stato ritornato un errore verra raccolto e gestito qui
	let err = determine_path('s');
	if err == Err(String::from("Wrong input!")) {println!("{:?}",err)};
}
```
- **Output**: `Wrong input!`
	
	
---
## **§ Annotazioni Esplicite del Tipo**
	
### Specifiche
	
- **Uso**: Utilizzo della sintassi `let variabile: tipo = valore`  per limitare e rendere più solido l'utilizzo della memoria ottimizzandola.
- **Tags**: #Types 
- **Esempio**:
	
```Rust
// Implicit Annotiation
let positive_number = 10; // Generates space in memory even for Negative numbers!

// Explicit Anntotation
let positive_number: u8 = 10; // Exclusively for Positive numbers!
```
	
### Generiche
	
- **Uso**: Si può utilizzare la annotazione anche per i tipi composti e complessi `enum`, `struct`, `vec`.
- **Tags**: #Types #Vectors #Enums #Structs 
- **Esempio**:
	
```Rust
enum DirectionalArrows {
	Up,
	Down,
	Left,
	Right,
}

struct Block {
	hash_sign: String,
	merkle_root: String,
	nonce: String,
}


fn main () {
	// ENUMERATIONS
	let run: DirectionalArrow = DirectionalArrows::Up;
	
	// STRUCTURES
	// Blockchain example just to be fancy
	let first_block: Block = {
	    hash_sign: String::from("tBJAihCD4Zc6++SXhcIEn3AqDzfm0x1XrqAa+DmzQh0="),
		merkle_root: String::from("CnPC478PrYguuMwDiaU6BpvC7bTWUMcll+BYhSN2e5k="),
		nonce_count: String::from("00000000000000000"),
	};
	let second_block: Block = {
		hash_sign: String::from("CnPC478PrYguuMwDiaU6BpvC7bTWUMcll+BYhSN2e5k="),
		merkle_root: String::from("sJVxRwy67sCy44Fzen5CT3wmi565pLkuKuqXqcwXkhc="),
		nonce_count: String::from("00000000000000000"),
	};
	
	// VECTORS
	let blockchain: vec<Block> = vec![first_block, second_block]; // STRUCTURES
	let dance: vec<DirectionalArrow> = { // ENUMERATIONS
		DirectionalArrows::Up,
		DirectionalArrows::Left,
		DirectionalArrows::Right,
		DirectionalArrows::Down,
	}

}
```
	
	
---
## **§ Traits**
	
### Implementazione
- **Uso**:  Definendo una funzione classica, sei legato ai tipi specifici definiti nella firma. Implementando traits, crei una sorta di "whitelist" di tipi che rende l'uso delle funzioni più versatile e dinamico, permettendo una maggiore riusabilità e flessibilità dopo la rottura di cazzo iniziale di implementazione della funzionalità in tema.
- **Sintassi**: `trait`
- **Tags**: #Types #Traits #Structs #Impl
- **Esempio**:
	
```Rust
struct Kid {
	age: u8,
	name: String
}
struct Adult{
	age: u8,
	name: String	
}

impl Kid {
	fn new (a: u8, n: String) -> Self {
		Self {
			age: a,
			name: n
		}
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}

impl Adult {
	fn new (a: u8, n: String) -> Self {
		Self {
			age: a,
			name: n
		}
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}

fn main () {
	let me = Adult::new(30, String::from("John")); 
	let him = Kid::new(10, String::from("Tom"));
	
	me.tell_name();
	him.tell_name();
}

```
	
Si converte il blocco con `trait`
	
```Rust
struct Kid {
	age: u8,
	name: String
}
struct Adult{
	age: u8,
	name: String	
}

trait NewPerson {
	fn new(n: String, a: u8) -> Self;
	fn tell_name(&self);
}

impl NewPerson for Kid {
	fn new(n: &str, a: u8) -> Self {
		Self {
			age: a,
			name: n.to_owned(),
		}
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}
impl NewPerson for Adult {
	fn new(n: &str, a: u8) -> Self {
		Self {
			age: a,
			name: n.to_owned(),
		}
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}
	
fn main () {
	let me = Adult::new("Kenneth", 27);
	let him = Kid::new("Costantino", 10);
	
	me.tell_name();
	him.tell_name();
}
```
	
>Osservando bene non sembrerebbe cambiato molto anzi utilizzando il trait abbiamo addirittura più linee di codice.
>	
>La vera potenza d'uso sta nel prossimo esempio ipotizzando una funzione che gestisce i *traits*
	
```Rust
struct Kid {
	age: u8,
	name: String
}
struct Adult{
	age: u8,
	name: String	
}

trait PersonTrait {
	fn tell_age(&self);
	fn tell_name(&self);
}

impl PersonTrait for Kid {
	fn tell_age(&self) {
		println!("Age: {}", &self.age);
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}
impl PersonTrait for Adult {
	fn tell_age(&self) {
		println!("Age: {}", &self.age);
	}
	fn tell_name(&self) {
		println!("Name: {}", &self.name);
	}
}
	
fn handle_crowd(anybody: impl Person) {
	/*
	L'elemento `anybody` è considerabile come qualunque tipo stia implementando  
	il trait Person grazie a `impl Person for Struct` 
	lasciando alla funzione la possibilità di ricevere più di un tipo 
	in questo caso sia Adult che Kid 
	
	Su `anybody` è possibile fare tutti i controlli del caso prima della chiamata
	delle funzioni proprietarie del `trait`
	*/
	anybody.tell_name(); // Chiamata diretta per semplicità
	anybody.tell_age(); // Chiamata diretta per semplicità
}
	
fn main () {
	let me = Adult{name: "Kenneth".to_owned(), age: 27};
	let him = Kid{name: "Costantino".to_owned(), age: 10};
	
	handle_crowd(me);
	handle_crowd(him);
	
}
```
	
>La funzione ipotetica `handle_crowd` lascia spazio non solo ad una ricezione dinamica di più tipi ma anche la possibilità di manipolare i tipi stessi all interno di se stessa.
	
### Default
	
- **Uso**:  Creare strutture ed enumerazioni con un valore di default per facilitare l implementazione di elementi regolabili in futuro o sotto condizione
- **Sintassi**: `impl Default for Struct`
- **Tags**: #Types #Traits #Structs  #Enums #Default #Impl
- **Esempio**:
	
```Rust
struct Tv {
	volume: u8,
	brightness: u8
}

impl Tv {
	fn new(v: u8, b:u8) -> Self {
		Self{volume: v, brightness: b}
	}
}

impl Default for Tv {
	fn default() -> Self {
		Self{volume: 10, brightness: 50}
	}
}

fn main() {
	let mivar = Tv::default(); // Call
}
```
	
	
### Funzioni Generiche
	
Utilizzando le precedenti componenti arriviamo all combinazione più utile le funzioni generiche
	
- **Uso**: Possibilità di creare funzioni che ricevono dei *traits* specifici.
- **Sintassi**: 
	
```Rust
fn handle_crowd<T, U, V, ... >(a: T, b: U, c: V)
where 
	T: Trait1,
	U: Trait2 + Trait3,
	V: Trait4 + Trait5 + Trait6
{
	//body
}
```

>Questo blocco è una firma complessa ed è il constraint della funzione, una white list per i traits accettati. Differente dalla forma generica :
	`fn handle_crowd(anybody: impl Person)` Che seleziona rispetto l'implementazione .
>
>I *traits* si indicano dalla letterati T in poi per convenzione e si specificano subito dopo la firma con la "firma ausiliare" `where` dove si assegna il *trait* alla lettera in maniera anche composta (`+`) .
	
- **Tags**: #Functions #Traits #Default #Impl
- **Comparazione**:
```Rust
fn handle_crowd(anybody: impl PersonTrait)

fn handle_crowd<T>(anybody: T)
where 
	T: PersonTrait,
```
- **Esempio**:
	
```Rust
struct Monitor {
	brand: String,
	volume: u8,
	brightness: u8
}
struct Tv {
	brand: String,
	volume: u8,
	brightness: u8
}
struct SmartTv{
	brand: String,
	volume: u8,
	brightness: u8
}

trait Setup {
	fn default () -> Self;
	fn brand(&self);
}

impl Setup for Tv {
	fn default () -> Self {
		Self{
			volume: 10,
			brightness: 50
		}
	}
	fn brand(&self){println!("{}",self.brand);}
}
impl Setup for SmartTv {
	fn default () -> Self {
		Self{
			volume: 15,
			brightness: 55
		}
	}
	fn brand(&self){println!("{}",self.brand);}
}


fn increase_volume_of <T> (a: &mut T, v: u8)
where 
	T: Setup,
{
	a.brand()
	a.volume = v;
}

fn main() {
	let mut mivar = Tv{brand: "Mivar".to_owned(),volume:0, brightness: 0};
	let mut pear_tv = SmartTv{
						brand: "Pear Tv".to_owned(), 
						volume:0, 
						brightness: 0
					};
	let mut simsamsung = Monitor{
							brand: "Simsamsung".to_owned(), 
							volume:0, 
							brightness: 0
						};
	
	increase_volume_of( &mut mivar, 10); // Output: Mivar
	increase_volume_of( &mut pear_tv, 10); // Output: Pear Tv
	// Non per Simsamsung perché non implementa il trait
}
```
	
- ##### Teoria della Monomorfizzazione
	E' un concetto autogestito dal compilatore di rust che crea concettualmente versioni di funzione dalla funzione di default nel seguente modo:
	
	```Rust
	// Quando Tv e SmartTv hanno trait Setup
	
	fn increase_volume <T> (a: T, v: u8)
	where 
		T: Setup,
	
	fn increase_volume(a: Tv, v: u8)
	fn increase_volume(a: SmartTv, v: u8)
	```
	
- ##### Sintassi
	Le funzioni generiche possono lavorare con tipi differenti preassegnati in maniera costrittiva
		
	**Esempio**:
	```Rust
	fn func(param: impl Trait){/*Copo funzione*/}
	
	fn func<T: Trait>(param: T){/*Copo funzione*/}
	
	fn func<T>(param: T) 
	where
		T: Trait
	{/*Copo funzione*/}
	```
	
### Struct Generiche
	
- **Definizione**: Le strutture generiche permettono la gestione dei campi in modo dinamico rispetto il trait.
- **Tags**: #Structs #Traits 
- **Esempio**:
	
```Rust
struct Structure<T: Trait> {
	field: T,
}
```
	
	
---
##### Progressione Suggerita
[Rust CheatSheet - Macro](rust-macros-cheatsheet.md)
[Rust CheatSheet - Dinamiche del codice](rust-controls-dynamics-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini

