## **Rust CheatSheet - Semplificare la Gestione Dati**
##### **Table of Contents**
###### [§ Ownership](#-Ownership-1) 🪪
###### [§ References](#-References-1) 🏷️
- [Trait Objects](#Trait-Objects)
###### [§ Lifetimes](#-Lifetimes-1) ⏱️
- [Funzioni](#Funzioni)
###### [§ Type Alias](#-Type-Alias-1) 🥸
- [Lifetimes](#Lifetimes)
###### [§ Type State](#-Type-State-1)
###### [§ Custom Errors](§-Custom-Errors-1) ⚠️
- [Display](#Display)
- [Error Crate](#Error-Crate)
###### [§ Closure Argument](#-Closure-Argument-1) 🌠
- [Dynamic Closures Arguments](#Dynamic-Closures-Arguments)
###### [§ Map Combinator](#-Map-Combinator-1) 🗺️
- [Collect](#Collect)
###### [§ Option Combinator Pattern](#-Option-Combinator-Pattern-1)❓
- [Is some or none](#Is-some-or-none)
- [Map](#Map)
- [Filter](#Filter)
- [Or Else](#Or-Else)
- [Unwrap Else](#Unwrap-Else)
###### [§ Using Collections Iterators](#-Using-Collections-Iterators-1)📚
- [Map](#Map-1)
- [Filter](#Filter-1)
- [Find](#Find)
- [Count](#Count)
- [Last](#Last)
- [Min Max](#Min-Max)
- [Take](#Take)
###### [§ Range](#-Range-1) 📏
- [Numeri](#Numeri)
- [Lettere](#Lettere)
	
---
## **§ Ownership**
	
La proprietà di una variabile è unica e fine a se stessa fuori da una funzione
    
- **Definizione**: La *Responsabilità* è un attributo atto alla gestione della memoria che garantisce la sicurezza dei dati.
- **Caso d' Uso**: Assicurazione su conflitti di concorrenza e sicurezza. I responsabili della proprietà sono: Funzioni, Closures, Structs, Enums and Scopes.
- **Tags**: #Ownership #Types #Strings #Management
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
	
	
---
## **§ References**
	
- **Definizione**: Attributo che permette di fare *riferimento* ad una variabile *senza* prenderne la *Responsabilità*
- **Sintassi**: `&`
- **Uso**: Prestito di dati *senza* vincoli di *Responsabilità*.
- **Tags**: #References #Types #Strings #Management #Borrowing
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
- **Tags**: #Management #Traits #References 
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
- **Tags**: #Management #Lifetimes #Borrowing 
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
## **§ Type Alias**
	
- **Descrizione**: Rinominare i tipi attraverso l' assegnazione classica `=`.
- **Uso**: Utile per identificare i tipi in base al loro uso.
- **Sintassi**: `type`
- **Tags**: #Types #Alias
- **Esempio**: 
	
```Rust
struct Person {
	name: String,
	age: u8,
}
struct Hospitalization {
	day: String,
	illness: String,
}

type Id = Person;
type Patient = HashMap<Id,Hospitalization>;

```
	
### Lifetimes
	
```Rust
type BorrowedItem<'a> = Vec<&'a str>
type GenericItem<T> = Vec<Item<T>>
```
	
	
---
## **§ Type State**
	
- **Definizione**: E' possibile utilizzare tipi personalizzati per creare degli stati strutture complesse aventi più forme, un po come le varianti di un enumerazione.
- **Uso**: Rappresentare uno stato in campi o variabili. 
- **Tags**: #Types #Management #Custom
- **Esempio**:
	
```Rust
struct Person<State>{
	name: String,
	condition: State,
}
struct Employed{
	company: String,
}

impl Person<Employed>{
	fn for_who(n: String, forw: String) -> Self { 
		Self { 
			name: n, 
			condition: Employed { company:forw }, 
		} 
	} 
}

fn main(){
	// Crea una persona disoccupata con for_who,
	let mut me = Person::for_who("Kenneth".to_owned(), "For you".to_owned());
	
}
```
	
##### Approfondimento Avanzato
	
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
	fn from_when(n: String, f: String) -> Self { 
		Self { 
			name: n, 
			condition: Unemployed { from: f }, 
		} 
	} 
}
impl Person<Employed>{
	fn for_who(n: String, forw: String) -> Self { 
		Self { 
			name: n,  
			condition: Employed { company:forw }, 
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
	
	
---
## **§ Custom Errors**

- **Definizione**: Gli errori personalizzati sono costruiti attraverso l'uso delle enumerazioni, devono implementare con `derive` il trait di `Debug`, `Error` e `Display`.
- **Uso**: Garantire una comprensione migliore delle eventuali problematiche che potrebbero verificarsi nel codice raggruppandoli per tipo. Preferibile rispetto ai messaggi string, con lo statement `match` i custom 
- **Tags**: #Management #Error #Traits #Display #Enums 
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
- **Tags**: #Display 
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
## **§ Closure Argument**
	
- **Descrizione**: È possibile passare una _closure_ come parametro di una funzione in Rust. Poiché le closure possono catturare variabili dall'ambiente e non hanno una dimensione fissa a tempo di compilazione, devono essere inserite all'interno di un `Box` per gestire la loro allocazione.
- **Uso**: Utile per passare alla funzione un modello da eseguire nel suo corpo, consentendo di definire un comportamento flessibile.
- **Sintassi**: `Box<Fn(Type,Type, ... ) -> Type>`
- **Tags**: #Functions #Closures #Boxing #Heap 
- **Esempio**:
	
```Rust
fn mathematics( a: i32, b: i32, operation: Box<Fn(i32,i32)-> i32> ) -> i32 {
	a + b * opreration(a,b)	
}
	
fn main() {
	let add_closure = |a, b| a + b;
	let add_boxed: Box<_> = Box::new(add_closure);
	
	println!("{}", mathematics(3,6,add_boxed))
}
```
- **Output**: `81`
	
### Dynamic Closures Arguments
	
- **Descrizione**: E' possibile passare nella firma di una funzione più di una *closure* consentendo la flessibilità di accettare diversi comportamenti dinamici.
- **Uso**: Utile a passare alla funzione più modelli da eseguire nel suo corpo.
- **Sintassi**: `dyn`
- **Tags**: #Functions #Closures #Boxing #Heap
- **Esempio**:
	
```Rust
fn mathematics( a: i32, b: i32, operation: Box< dyn Fn(i32,i32)-> i32> ) -> i32 {
	a + b * opreration(a,b)	
}
	
fn main() {
	let add_closure = |a, b| a + b;
	let sub_closure = |a, b| a - b;
	let add_boxed: Box<_> = Box::new(add_closure);
	let sub_closure: Box<_> = Box::new(sub_closure);
	
	println!("{},", mathematics(3,6,add_boxed))
	print!(" {}", mathematics(3,6,sub_closure))
}
```
- **Output**: `81, 27`
	
	
---
## **§ Map Combinator**

- **Descrizione**: Il Mapping è la pratica di  trasformare qualcosa in un altra. La funzione complementare `.map()` in Rust è lo strumento atto a questo scopo che restituisce un iteratore sui dati trasformati, pronti per essere consumati quando necessario.
- **Uso**: La funzione `.map()` serve ad applicare Closures( Funzioni semplici atte a piccole modifiche ) in maniera dinamica quindi su risultati di funzione funzione().map(closure) multivalore quindi inaspettati o collezioni di dati, come per formattare in upcase dati che si generano via via che la funzione primaria genera dati per .map()
- **Tags**: #Map #Combinators
- **Esercizio**:
	
```Rust
// Funzione di somma con input che resituisce un un i8 o un Messaggio di errore
fn sum (a:i8, b:i8, input: Option<i8>) -> Result<i8, String> {
    match input {
        Some(i) => Ok(a + b + i),
        None => Err(String::from("No input!"))
    }
}
  
/*
Vogliamo aggiungere al risultato di sum() una correzione.
Map è dinamico nel caso in cui sum() restituisca un messaggio di errore
.map() non eseguirà la closure.
*/
fn main () {
    let input = Some(1);
    let correction = 6;
    let sum_corrected = sum(1,1,input).map(|sum_result| sum_result + correction);
  
    match sum_corrected {
        Ok(sum_r) => println!("Result: {}", sum_r),
        Err(e) => println!("Error: {}", e)
    }
}
```
- **Output**: `9`
- `None` **Output**: `No Input!`
	
### Collect
	
- **Descrizione**: Il collettore *collect* trasforma un iteratore in una collezione, cosa che può essere necessaria a seconda di come intendi utilizzare i dati.
- **Sintassi**: `.collect()`
- **Caso d' Uso**: Collect finalizza i dati elaborati, se scopo è semplicemente passare i dati trasformati a un altro passo di elaborazione o consumare i risultati una sola volta, potresti non aver bisogno di usare `collect()`.
- **Esercizio**:
	
```Rust
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
	let uppercase_names: Vec<String> = names.iter()
											.map(|s|-> String {s.to_uppercase()})
		                                    .collect();
    println!("{:?}", uppercase_names);
```
- **Output**: `["KENNETH", "PHILIP", "LINDA", "GIULIA"]`
	
	
---
## **§ Option Combinator Pattern**

- **Descrizione**: Lista dei combinators per i tipi Option (Vedi: **§ Tipi Aggiuntivi | Option** in [Rust Cheat Sheet - Tipi](rust-types-cheatsheet.md)).
- **Tags**: #Option #Combinators #Closures 
	
### Is some or none
	
- **Descrizione**: Restituiscono dati booleani sulla veridicità del nome del combinatore
- **Tags**: #Some #None
- **Esempio**:
	
```Rust
let opt: Option<i8> = Some(3) ;

let opt_is_some = opt.is_some() ; // Output: true
let opt_s_none = opt.is_none() ; // Output: false

```
	
### Map
	
- **Descrizione**: Itera i dati solo se la Option è `Some(_)` in questo caso `Some(3)` sarà num
- **Tags**: #Map 
- **Esempio**:
	
```Rust
let opt: Option<i8> = Some(3) ;

let opt_mapped = opt.map(|num|num + 6) ; //Output: Some(9)
```
	
### Filter
	
- **Descrizione**: Opera con una *closure* di comparazione che usa il *borrowing* su controlli `if` per filtrare ottenendo gli elementi specificati
- **Tags**: #Filter
- **Esempio**:
	
```Rust
let opt: Option<i8> = Some(3) ;

let comparison = 3 ;
let opt_filtered = opt.filter(|num|num == &comparison) ;
// num = 3 Output: 3 , num != 3 Output: None
```
	
### Or Else
	
- **Descrizione**: Controlla se `opt` possiede dati se falso ne assegna uno `Some(9)`
- **Tags**: #Or #Else  
- **Esempio**:
	
```Rust
let opt: Option<i8> = Some(3) ;

let opt_or_else = opt.or_else(||Some(9)) ; // Output: Some(3);
```
	
### Unwrap Else
	
- **Descrizione**: Utilizza un valore default se `opt` è `None` 
- **Tags**: #Unwrap #Else 
- **Esempio**:
	
```Rust
let opt: Option<i8> = Some(3) ;

let default_val = 9; 
let unwrapped = opt.unwrap_or_else(|| default_val);
// Output: unwrapped == 3 if opt == None unwrapped == 9
}
```
	
> Controllare la libreria standard di Rust  per maggiori funzionalità( Vedi: **§ Standard Library API Docs** in [Rust Cheat Sheet - Elementi base](rust-basics-cheatsheet.md).)
	
	
---
## **§ Using Collections Iterators**
	
- **Definizione**: Un iteratore è una struttura che ispeziona gli elementi di una collezione di dati permettendo ad un *combinator* di essere eseguito.
- **Sintassi**: `.iter()`
- **Uso**: Ridurre la sintassi permettendo un codice più semplice da leggere.
- **Tags**: #Iterators #Combinators #Vectors #Arrays 
	
### Map
	
- **Tags**: #Map 
- **Esempio**:
	 
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Shark".to_owned(),
] ;

// Soluzzione con ciclo for
/*
let mut veterinary_list: Vec<String> = vec![] ;
for anml in animals {
	healedanimal(anml)
	veterinary_list.push(String::from(anml) + " Rescued!")
}
*/

// Soluzione con iteratore
/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Si converte anml in String da &String si concatena &str
[3] Si colleziona l' Iteratore per finalizzare l' ispezione
*/
let veterinary_list: Vec<String> = animals
	.iter() // [1]
	.map(|anml| String::from(anml) + " Rescued!") // [2]
	.collect(); // [3]
		
println!("To be treated: {:?}",animals);
println!("Treated: {:?}",veterinary_list);
```
-  **Output**: 
`To be treated: ["Cat", "Lion", "Dog", "Shark"]`
`Treated: ["Cat Rescued!", "Lion Rescued!", "Dog Rescued!", "Shark Rescued!"]`
	
### Filter
	
- **Tags**: #Filter
- **Esempio**:
	
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Shark".to_owned(),
	"Cat".to_owned(),
] ;

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Comparazione stringa
[3] Si colleziona l' Iteratore per finalizzare l' ispezione
*/
let veterinary_list_filtered: Vec<&String> = animals
	.iter() // [1]
	.filter(|anmlf| **anmlf == "Cat".to_owned()) // [2]
	.collect(); // [3]
		
println!("To be Filtered: {:?}",animals);
println!("Filtered: {:?}",veterinary_list_filtered);
```
-  **Output**: 
`To be Filtered: ["Cat", "Lion", "Dog", "Shark", "Cat"]`
`Filtered: ["Cat", "Cat"]`
	
### Find
	
- **Descrizione**: Cerca un elemento specifico della collezione ritornando un `Option`.
- **Tags**: #Find
- **Esempio**:
	
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Shark".to_owned(),
	"Cat".to_owned(),
] ;

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Comparazione stringa, non necessita di .collect()
*/
let veterinary_list_found: Option<&String> = animals
	.iter() // [1]
	.find(|anmlf| **anmlf == "Cat".to_owned()); // [2]
		
println!("To be Found in: {:?}",animals);
println!("Found: {:?}",veterinary_list_found);;
```
-  **Output**: 
`To be Found in: ["Cat", "Lion", "Dog", "Shark", "Cat"]`
`Found: Some("Cat")`
	
### Count
	
- **Descrizione**: Incredibilmente conta i componenti della collezione.
- **Tags**: #Count
- **Esempio**:
	
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Shark".to_owned(),
	"Cat".to_owned(),
] ;

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Conta elementi collezione, non necessita di .collect()
*/
let veterinary_list_count = animals
	.iter() // [1]
	.count(); //[2]
		
println!("To be counted: {:?}",animals);
println!("Number of elements: {:?}",veterinary_list_count);
```
-  **Output**: 
`To be Counted: ["Cat", "Lion", "Dog", "Shark", "Cat"]`
`Number of elements: 6`
	
### Last
	
- **Descrizione**: Incredibilmente trova l' ultimo valore restituendo un `Option` essendo i vettori inizializzabili vuoti. 
- **Tags**: #Last
- **Esempio**:
	
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Cat".to_owned(),
	"Shark".to_owned(),
] ;

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Ottiene ultimo elemento della collezione, non necessita di .collect()
*/
let veterinary_list_last: Option<&String> = animals
	.iter() // [1]
	.last(); // [2]
		
println!("What is last in: {:?}",animals);
println!("Last item: {:?}",veterinary_list_last);
```
-  **Output**: 
`What is last in: ["Cat", "Lion", "Dog", "Cat", "Shark"]`
`Last item: Some("Shark")`
	
### Min Max
	
- **Descrizione**: Incredibilmente Minimo o Massimo della collezione restituendo un `Option` essendo i vettori inizializzabili vuoti. 
- **Tags**: #Min #Max
- **Esempio**:
	
```Rust
let numbers: Vec<i32> = vec![0,3,-12,6,-9,96,];

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Minimo o Massimo preesente, non necessita di .collect()
*/
let number_list_min: Option<&i32> = numbers
	.iter() // [1]
	.min(); // [2]
	
let number_list_max: Option<&i32> = numbers
	.iter() // [1]
	.max(); // [2]
		
println!("Numbers: {:?}",animals);
println!("Min: {:?}",number_list_min);
println!("Max: {:?}",number_list_max);
```
-  **Output**: 
`Numbers: [0, 3, -12, 6, -9, 96]`
`Min: Some(-12)`
`Max: Some(96)`
	
### Take
	
- **Descrizione**: Incredibilmente prende un valore specifico in considerazione
- **Tags**: #Take
- **Esempio**:
	
```Rust
let animals: Vec<String> = vec![
	"Cat".to_owned(),
	"Lion".to_owned(),
	"Dog".to_owned(),
	"Shark".to_owned(),
	"Cat".to_owned(),
] ;

/*
[1] usa iter() per ottenere riferimenti ai dati, quindi anml è &String
[2] Prende le prime .take(n) posizioni
[3] Si colleziona l' Iteratore per finalizzare l' ispezione
*/
let veterinary_list_take: Vec<&String> = animals
        .iter() // [1]
        .take(3) // [2]
        .collect(); // [3]
		
println!("To be choose: {:?}",animals);
println!("Taken: {:?}",veterinary_list_take);
```
-  **Output**: 
`To be Taken: ["Cat", "Lion", "Dog", "Shark", "Cat"]`
`Taken: ["Cat", "Lion", "Dog"]`
	
> E' possibile concatenare queste funzioni insieme al fine di creare iteratori complessi
	
	
---
## **§ Range**
	
**Descrizione**: E' possibile prendere in considerazione una serie di elementi in sequenza
**Sintassi**: `n..m`, `n..=m`
**Tags**: #Range #Integers #Float #Char
	
### Numeri
	
- **Esempio**:
	
```Rust
fn main () {
	let range = 1..3; // Escluso
	let range = 3..=9; // Compreso =n

	for num in 1..10 {
		print!("{}", num)
	}

}
```
-  **Output**: `1,2,3,4,5,6,7,8,9`
	
### Lettere
	
- **Esempio**:
	
```Rust
fn main () {
	for ch in 'a'..='k' {
		print!("{}", ch)
	}
}
```
-  **Output**: `a,b,c,f,e,f,g,j,k,`
	
	
---
	
>E' consigliato usare i `Tags` in relazione a gli altri Cheatsheets per un quadro sull'argomento più esaustivo.
##### Progressione Suggerita
[Rust CheatSheet - ](.md)
	
---
	
**Author:** Kenneth Boldrini