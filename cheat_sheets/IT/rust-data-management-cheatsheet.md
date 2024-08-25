## **Rust CheatSheet - Semplificare la Gestione Dati**
##### **Table of Contents**
###### [§ Closures](#-Closures-1) ⚡
- [Basics](#Basics)
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
##### [§ Advanced Control Expressions](#-Advanced-Control_Expressions-1) 🛂
- [While let](#While-let)
- [If Let Else](#If-Let-Else)
###### [§ Modules](#-Modules-1) 🧩
###### [§ Project Structure](#-Project-Structure-1) 🧱
- [Cargo](#Cargo)
- [External Crates - Libraries](#External-Crates---Libraries)
---
## **§ Closures**
	
- **Descrizione**: Le closure sono Funzioni semplici senza identità atte a piccole modifiche. Per differenziare le closures da le funzioni classiche si usa i pipes `| ... |` come utilizziamo le parentesi tonde `( ... )` . Possono inoltre auto definire i loro parametri della loro firma nella forma breve.
- **Tags**: #Closures 
- **Sintassi**: 
```Rust
// Firma Estesa
let closure_name = | a: Type, b: Type | -> Return_Type { ... } ;
// Firma Breve
let closure_name = | a , b | ... ;
```
	
### Basics
	
- **Uso**: Si può evitare grosse parti di sintassi utilizzando la forma breve della closure. E possiamo utilizzarle in maniera strategica in situazioni dinamiche, come potrai vedere nelle sezioni (§) di seguito.
- **Esempi**:
	
```Rust
fn main () {
	// Firma e corpo esteso
	let sub = |a: i64, b: i64| -> i64 { a + b };
	// Firma e corpo breve
	let add = |a,b| a + b;
	let x = add(1,1);
	let y = sub(x,1);

	println!("{}",y)
}
```
- **Output**: `1`
	
	
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
## **§ Modules**
	
**Descrizione**: I moduli in Rust sono utilizzati per raggruppare funzioni, definizioni di tipo, implementazioni e altri moduli. Funzionano come spazi dei nomi e come unità di organizzazione del codice, consentendo la privacy del codice e la riutilizzabilità. Ogni modulo può essere considerato come un file separato.
**Sintassi**: `mod nome_modulo {_}`, `use nome_modulo::_`
**Tags**:  #Modules #Functions 
**Esempio**:
	
```Rust
mod connection {
	pub fn init (_) {_}
	pub fn abort (_) {_}
	fn check (_) {_} // Funzione privata
}
mod order {
	pub fn sell (amount) {_}
	pub fn buy (amount) {_}
}

fn main () {
	// Import selettivo del modulo per l'utilizzo in funzione
	use connection::init;
	init();
	connection::abort();
	// check() non è possbile usarla perchè è privata
	
	// Import totale del modulo per l'utilizzo in funzione
	use order::*
	buy(10);
	sell(9);
}
```
	
	
---
## § Project Structure
	
- **Definizione**:
	
```
progetto_rust/
│
├── Cargo.toml   # File cgf Cargo, dipendenze e le impostazioni del progetto
├── Cargo.lock   # File Cargo per tracciare le versioni esatte delle dipendenze
│
├── src/   # Directory contenente i sorgenti del progetto
│   ├── lib/      # Directory contenente elementi di supporto alla funzionalità
│   │    └── mod.rs        # File di contenimento dei moduli esterni
│   ├── main.rs            # File punto di ingresso dell'applicazione
│   └── lib.rs             # File punto di ingresso della libreria 
│
├── tests/                    # Directory per i test di integrazione
│   └── integration_test.rs   # Esempio di test di integrazione
│
├── examples/              # Esempi di codice che dimostrano l'uso della libreria
│   └── simple.rs          # Esempio semplice
│
├── benches/          # Directory Benchmark 
│   └── performance.rs     # File di benchmark per valutazione di parti del codice
│
└── target/                # Directory generata dove Cargo compila il progetto
│
└── .gitignore             # File di configurazione Git
```
	
### Cargo
	
- **Descrizione**: Cargo è il sistema di setup delle dependencies, metadata, configurazione build, gestione del workspace e features opzionali.  E' il cuore della della configurazione dei progetti.
- **Tags**: #Cargo
- Esempio:
	
```sh
# Nella cartella progetto
cargo init .

# In posizione relativa
cargo init path
```
	
```sh
# Nel terminal il link che porta alle possibili implementazioni del file .toml

note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```
	
##### .toml
	
- **Descrizione**: File di configurazione degli elementi di cargo.
- **Uso**: All' URL *crates.io* si trovano i crates importabili nella sezione `[dependencies]` come lista dei requisiti. 

```sh
# Aggiundere automaticamente un crate tra le dependencies
cargo add create_name
```

- **Tags**: #Toml
- **Esempio**:
	
```Rust 
[package]
name = "ProjectName"
version = "VersionNumber"
authors = "[AuthorName <email>]"
edition = "Year"

[dependencies]
crate_name = "VersionNumber"

[lib] // Inserimento moduli esterni
name = "lib"
path = "src/lib/lib.rs"
```
	
```Rust
// src/lib/lib.rs is a Collection of modules of the project 

pub mod some_module;

```
	
> Si controlla la documentazione del cargo su `cargo.io` per l'implementazione nel `main.rs`.
> Nel codice si importerà il crate con `use crate_name::needed_element`.
	
### External Crates - Libraries
	
- **Descrizione**: I crates esterni sono librerie che possono essere incorporate nei progetti Rust attraverso Cargo. Possono contenere la compartimentazione delle funzioni del codice secondo criteri di convenienza tramite la creazione di .rs, in cui possono essere istanziati moduli funzioni strutture ed elementi di rust in maniera pubblica con l'uso di `pub` al fine di modularizzare e mantenere il codice pulito e manutenibile.
- **Sintassi**: `extern crate` `use`
- **Tags**: #Crates #ExternalLibraries
- **Esempio**:
	
```Rust
// src/lib/some_module.rs 

pub struct Person { 
	pub name: String, 
	pub age: u32, 
}

pub fn some_function (_) {_}
```
	
##### Accessibilità
	
```Rust
// Importazione di un crate esterno e utilizzo nel progetto 
use serde_json::json; 

// Parola chiave `super` per accedere ai moduli al livello superiore 
use super::config::Settings; 
// Accede a `Settings` definito in un modulo 'config' un livello sopra 

// Importazione diretta di strutture o funzioni da moduli interni 
use crate::auth::validate_credentials; // Dove `auth` è un modulo nel progetto
```
	
	
---
##### Progressione Suggerita
[Rust CheatSheet - ](.md)
	
---
	
**Author:** Kenneth Boldrini