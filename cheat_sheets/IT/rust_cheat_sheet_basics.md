# Rust Cheat Sheet - Elementi base
Elementi base per esecuzione di rust
	
---
## **Basics**
	
- ##### Istanziazione Variabile
	
	Sintassi: `let` (`;`)
	
	```Rust
	//snake_case
	let my_variable: u32 = 10 ;
	```
	
- ##### Istanziazione Variabile Mutabile
	
	Sintassi: `let mut` (`;`)
	
	```Rust
	//snake_case
	let mut my_variable: bool ;
	my_variable = false ;
	```
	
- ##### Struct
	
	Sintassi: `struct`
		
	```Rust
	// PascalCase
	struct Struct {
		...
	}
	```
	
- ##### Enum
	
	Sintassi: `enum`
	
	```Rust
	// PascalCase
	enum MyEnum {
		...
	}
	```
	
- ##### Chiamata d'ispezione
	
	Sintassi: `::`
	
	Descrizione: Servono per chiamare i metodi associati di una struct, accedere agli elementi all'interno di moduli.
	
``` Rust
	Vec::new()
	String::new()
```
	
- ##### Istanziazione Funzione
	
	Sintassi: `fn`
	
	```Rust
	//snake_case
	fn my_function (arg1: u32, arg2: u8) -> u32{
		...
	}
	```
	
- ##### Commenti e testi
	
	Sintassi: `//`
	
	```Rust
	// Comment
	
	/// Documentation!
	
	/*
	Comment,
	Block!
	*/
	```
	
	Sintassi: `\`
	
	```Rust
	print!(
		"Multi \
		lined \ 
		string"
	);
	```
	
- ##### Variabili inutilizzate
	
	Sintassi: `_`
	
	```Rust
	/*
	Per questioni di chiarezza rispetto il compilatore, 
	per le variabili inutilizzate si aggiunge `_`
	*/
	
	let _x = 10 ;
	let y = 9 ;
	
	print!("{}",y) ;
	```
	
- ##### Procedura di Compilazione ed Esecuzione
	
	Nella cartella dove si trova il `main.rs` :
	
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
##### Progressione Suggerita
[[rust_cheat_sheet_types]]
	
---
	
**Author:** Kenneth Boldrini
