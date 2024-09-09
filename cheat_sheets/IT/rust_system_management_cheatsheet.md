# **Rust CheatSheet - Gestione del Sistema**
##### **Table of Contents**

###### [§ Modules](#-Modules-1) 🧩
###### [§ Project Structure](#-Project-Structure-1) 🧱
- [Cargo](#Cargo)
- [External Crates - Libraries](#External-Crates---Libraries)
###### [§ Threads](#-Thread-1) 🧵
- [Channels](#Channels)
- [Bidirectional Communication](#Bidirectional-Communication)
	
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
## **§ Project Structure**
	
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
crate_name = "VersionNumber" // Oppure "*" per ultima versione disponibile

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
## **§ Threads**
	
- **Descrizione**: I **thread** sono esecuzioni parallele del programma principale. In Rust, un thread viene rappresentato da un tipo specifico: `JoinHandle<Type>`.
- **Uso**: I thread sono utili per eseguire task secondarie mentre il programma principale continua la sua esecuzione. Una variabile di tipo `JoinHandle` agisce come un "handle" (maniglia) che permette di controllare e sincronizzare il thread, attendendo il suo completamento tramite il metodo `.join()`.
- **Sintassi**: `thread::spawn()`
- **Tags**: #Threads
- **Esempio**:
	
```Rust
use std::thread;

let handle_variable = thread::spawn(move || {
	// Codice eseguito nel thread 
	println!("Esecuzione parallela in un thread");
});

handle_variable.join(); // .join() ritorna un type Result
```
	
> Se il `main()` finisce prima dell'esecuzione dei threads essi verranno interrotti, per prevenire ciò assicurarsi che i `.join()` corrispettivi siano correttamente posizionati nel codice è cruciale.
	
### Channels
	
- **Descrizione**: I canali permettono comunicazioni unidirezionali tra *threads*. Sebbene la libreria standard fornisca un modulo idoneo alla comunicazione tra threads, `crossbeam-channel` è un crate più completo.
	
```Rust
// Installazione diretta
[Dependencies]
crossbeam-channel = "*"
```
	
```sh
# Installazione via cargo
cargo install crossbeam-channel
```
	
- **Uso**: Anche se è possibile inviare qualsiasi tipo di messaggio è consigliabile l'utilizzo degli `enum` che garantiscono la personalizzazione delle versioni in maniera robusta, grazie anche all'utilizzo del `match` statement. I canali sono da considerare come dei buffer di informazioni da cui un "sender" di tipo Result `Sender<type>` invia informazioni ad un "receiver" di tipo `Receiver<type>`.
- **Sintassi**: `.send()` `.recv()`
- **Tags**: #Threads #Channels
- **Esempio**:
	
	```Rust
use crossbeam_channel::unbounded; // Crea un canale con buffer illimitato;

let (sender, receiver) = unbounded();

sender.send("Message")?; // ? essendo un sender e receiver dei Result type

use std::thread;

let my_thread_handle = thread::spawn(move || {
	match receiver.recv() {
		Ok(msg) => println!("{}",msg),
		Err(e) => println!("{}",e),
	}
})

my_thread_handle.join(); // Chiamata di chiusura del thread
	```
	
	 #### Messaggio Enum 
	 
	```Rust
use crossbeam_channel::unbounded; // Crea un canale con buffer illimitato;

enum Msg {
	PrintData(String),
	Calculate(i64,i64),
	Quit,
}

fn main(){
	let (sender, receiver) = unbounded();
	
	use std::thread;
	
	//thread
	let my_thread_handle = thread::spawn(move || loop{
		match receiver.recv() { // il ricevitore è dentro il thread 
			Ok(msg) => match msg {
				Msg::PrintData(m) => println!("{}",m),
				Msg::Calculate(a,b) => println!("{}",(a+b)),
				Msg::Quit => break, // Esce dal loop
			},
			Err(e) => {
				println!("{}",e)
				break
			},
		}
	})
	
	// Il mittente invia al corrispettivo ricevitore nel thread
	sender.send(Msg::Calculate(3,3)).unwrap(); 
	drop(sender) 
	/*
	In thread di `loop` infiniti si usa `drop()` che recide la connessione
	generando `Err` 
	*/
	// Aspetta che il thread termini 
	my_thread_handle.join().unwrap();
}
	```

	
	 #### Clone
	- **Descrizione**: Il sender e il receiver possono esser clonate per gestire il flusso di informazioni caricate nel buffer in maniera dinamica.
	- **Uso**: Utile alla sincronizzazione dei processi.
	- **Esempio**:
	
```Rust
use crossbeam_channel::unbounded; // Crea un canale con buffer illimitato;

let (sender, receiver) = unbounded();
let receiver_clone = receiver.clone();

sender.send("Message")?; // ? essendo un sender e receiver dei Result type

use std::thread;

let my_thread_handle1 = thread::spawn(move || {
	match receiver.recv() {
		Ok(msg) => println!("{}",msg),
		Err(e) => println!("{}",e),
	}
})

let my_thread_handle2 = thread::spawn(move || {
	match receiver_clone.recv() {
		Ok(msg) => println!("{}",msg),
		Err(e) => println!("{}",e),
	}
})

// Chiamate di chiusura del thread
my_thread_handle1.join(); 
my_thread_handle2.join(); 
```
	
### Bidirectional Communication 
	
- **Uso**: Questo meccanismo rende possibile la comunicazione bidirezionale al fine di avere un codice responsivo e dinamico. 
- **Tags**: #Threads #BidirectionalChannels 
- **Esempio**:
	
```Rust
use crossbeam_channel::unbounded; // Crea un canale con buffer illimitato;
use std::thread;

fn main(){
	let (thread_tx, thread_rx) = unbounded();
	let (main_tx, main_rx) = unbounded();
	
	thread_tx.send("Message").unwrap(); // `unwrap()` gestisce i Result
	
	let my_thread_handle = thread::spawn(move || {
		match thread_rx.recv() {
			Ok(msg) => {
				main_tx.send("Thread: Received").unwrap();
			},
			Err(e) => println!("Thread: {}",e),
		}
	})
	
	println!("main: {}",main_rx.recv().unwrap());
	
	// Chiamate di chiusura del thread
	my_thread_handle.join().unwrap(); 
}
```
	
	
---
	
>E' consigliato usare i `Tags` in relazione a gli altri Cheatsheets per un quadro sull'argomento più esaustivo.
##### Suggested Progression
[Rust CheatSheet - Funzioni](rust-functions-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini
