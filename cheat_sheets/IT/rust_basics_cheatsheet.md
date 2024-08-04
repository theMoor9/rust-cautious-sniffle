# **Rust Cheat Sheet - Elementi base**

##### **Table of Contents**
###### [§ Basics](#-Basics-1)
- [Istanziazione Variabile](#Istanziazione-Variabile)
- [Istanziazione Variabile Mutabile](#Istanziazione-Variabile-Mutabile)
- [Struct](#Struct)
- [Enum](#Enum)
- [Chiamata d'ispezione](#Chiamata-d'ispezione)
- [Import Librerie](#Import-Librerie)
- [Istanziazione Funzione](#IstanziazioneFunzione) 
- [Commenti e testi](#Commenti-e-testi)
- [Variabili Inutilizzate](#Variabili-Inutilizzate)
- [Procedura di Compilazione ed Esecuzione](#Procedura-di-Compilazione-ed-Esecuzione)
###### [§ Standard Library API Docs](#-Standard-Library-API-Docs) 
- [Procedura di apertura API Docs e ricerca](#Procedura-di-apertura-API-Docs-e-ricerca)
###### [§ Project Structure](#-Project-Structure-1)
- Cargo
- Src - main.rs
- Src - lib.rs
- 

#Basics 

---
## **§ Basics**
	
- ##### Istanziazione Variabile
	
	**Sintassi**: `let` (`;`)  
	**Posizione obbligatoria rispetto il codice**: *Dentro una funzione funzione*
	
	```Rust
	//snake_case
	let my_variable: u32 = 10 ;
	```
	
	
- ##### Istanziazione Variabile Mutabile 
	
	**Sintassi**: `let mut` (`;`)  
	**Posizione obbligatoria rispetto il codice**: *Dentro una funzione funzione*
	
	```Rust
	//snake_case
	let mut my_variable: bool ;
	my_variable = false ;
	```
	
	
- ##### Struct
	
	**Sintassi**: `struct`  
	**Tags**: #Structs  
	**Posizione obbligatoria rispetto il codice**: *Nessuna*
		
	```Rust
	// PascalCase
	struct Struct {
		...
	}
	```
	
	
- ##### Enum
	
	**Sintassi**: `enum`  
	**Tag**: #Enums   
	**Posizione obbligatoria rispetto il codice**: *Nessuna*
	
	```Rust
	// PascalCase
	enum MyEnum {
		...
	}
	```
	
	
- ##### Chiamata d'Ispezione 
	
	**Descrizione**: Servono per chiamare i metodi associati di una struct, accedere agli elementi all'interno di moduli.  
	**Sintassi**: `::`  
	
	``` Rust
	Vec::new()
	String::new()
	```
	
	
- ##### Import Librerie
	**Descrizione**: La dichiarazione per l'importazione delle librerie è `use`, seguita dai due punti (`::`) per importare gli elementi specifici della libreria ai fini della memoria.
	**Sintassi**: `use` (`;`)  

	```Rust
	use library::category::element_needed ;
	// Like
	use std::collection::HashMap ;
	```
	
	
- ##### Istanziazione Funzione 
	
	**Sintassi**: `fn`  
	**Posizione obbligatoria rispetto il codice**: *Nessuna*  
	
	```Rust
	//snake_case
	fn my_function (arg1: u32, arg2: u8) -> u32{
		...
	}
	```
	
	
- ##### Commenti e testi
	
	**Sintassi**: `//`  
	**Posizione obbligatoria rispetto il codice**: *Nessuna*  
	
	```Rust
	// Comment
	
	/// Documentation!
	
	/*
	Comment,
	Block!
	*/
	```
	
	**Sintassi**: `\`  
	
	```Rust
	print!(
		"Multi \
		lined \ 
		string"
	);
	```
	
- ##### Variabili Inutilizzate
	
	**Sintassi**: `_`  
	
	```Rust
	/*
	Per questioni di chiarezza rispetto il compilatore, 
	per le variabili inutilizzate si aggiunge `_`
	*/
	
	let _x = 10 ;
	let y = 9 ;
	
	print!("{}",y) ;

	// Tipo Unit (Tupla vuota) 
	let unit = (); 
	println!("{:?}", unit); 
	```
	- `Output: ()`
	
- ##### Procedura di Compilazione ed Esecuzione
	
	Tramite Terminal nella cartella dove si trova il `main.rs` :  
	
	1. **Compila**  
		```sh
	   $ rustc main.rs
	   ```
	2. Esegue **Windows**  
		```sh
	   $ .\main.exe
	   ```  
	3. Esegue **non-Windows** (Linux/macOS):  
		```sh
	   $ ./main
	   ```     
	
---
## § Standard Library API Docs
	
Come spesso accade, i linguaggi possiedono una vasta gamma di metodi e funzioni necessarie ad ampliare la versatilità delle capacità coding. Gli API Docs sono una risorsa indispensabile per ciò.
##### Procedura di apertura API Docs e ricerca
	
Tramite Terminal nella cartella dove si trova il `main.rs` :  

1. **Apertura Rust Library Documentation** 
   ```sh
   $ rustup doc
   ```
   
2. **Nella sezione "Use Rust" si trova il paragrafo "The Standard Library" che conterrà il link diretto alla documentazione API** 

3. **Una volta nell'API Documentation è possibile cercare qualsivoglia elemento necessario ad una corretta implementazione del proprio codice**

	
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
	
- **Descrizione**: Cargo è il sistema di setup delle dependencies, metadata, configurazione build, gestione del workspace e features opzionali.  
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
- **Tags**: #Toml
- **Esempio**:
	
```Rust 
[package]
name = "ProjectName"
version = "VersionNumber"
authors = "[AuthorName <email>]"
edition = "Year"

[dependencies]
cargo_name = "VersionNumber"

[lib] // Inserimento moduli esterni
name = "Module_Name"
path = "src/lib/mod.rs"

```
	
> Si controlla la documentazione del cargo su `cargo.io` per l'implementazione nel `main.rs`.
> Nel codice si importerà il crate con `use crate_name::needed_element`.
	
### External Crates
	
- **Descrizione**: Compartimentazione delle funzioni  del codice secondo criteri di convenienza tramite il file mod.rs, file di istanziazione dei moduli.
- **Tags**: #Crates
- **Esempio**:
	
```Rust
pub use module::
```

##### Accessibilità
	
- **Descrizione**: 
- **Esempio**:
	
```Rust
// super è Equivalente a cercare un elemento fuori dal file o dalla cartella 
use super::super::ext_crate_name;
```
	
---
##### Progressione Suggerita
[Rust Cheat Sheet - Tipi](rust_types_cheatsheet.md)
[Rust Cheat Sheet - Cargo](rust_cargo_cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini