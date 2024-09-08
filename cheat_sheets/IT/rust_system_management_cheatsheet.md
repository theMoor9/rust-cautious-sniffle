# **Rust CheatSheet - Gestione del Sistema**
##### **Table of Contents**

###### [§ Modules](#-Modules-1) 🧩
###### [§ Project Structure](#-Project-Structure-1) 🧱
- [Cargo](#Cargo)
- [External Crates - Libraries](#External-Crates---Libraries)
###### [§ Threads](#-Thread-1) 🧵
	
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
## **§ Threads**
	

	
	
---
	
>E' consigliato usare i `Tags` in relazione a gli altri Cheatsheets per un quadro sull'argomento più esaustivo.
##### Suggested Progression
[Rust CheatSheet - Funzioni](rust-functions-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini
