# **Rust CheatSheet - Elementi base**

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
	
#Basics 
	
---
## **§ Basics**
	
- ##### Istanziazione Variabile
	
	**Sintassi**: `let` (`;`)  
	**Posizione obbligatoria rispetto il codice**: *Dentro una funzione*
	
	```Rust
	//snake_case
	let my_variable = 10 ;
	```
	
	
- ##### Istanziazione Variabile Mutabile 
	
	**Sintassi**: `let mut` (`;`)  
	**Posizione obbligatoria rispetto il codice**: *Dentro una funzione*
	
	```Rust
	//snake_case
	let mut my_variable ;
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
	   
1. **Nella sezione "Use Rust" si trova il paragrafo "The Standard Library" che conterrà il link diretto alla documentazione API** 
	
1. **Una volta nell'API Documentation è possibile cercare qualsivoglia elemento necessario ad una corretta implementazione del proprio codice**
	
	
---
##### Progressione Suggerita
[Rust CheatSheet - Tipi](rust-types-cheatsheet.md)
[Rust CheatSheet - Cargo](rust-cargo-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini