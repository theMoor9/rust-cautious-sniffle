## **Rust Cheat Sheet - Cargo**
##### **Table of Contents**
###### [§ Documentazione](#-Documentazione-1)
- [Aggiungere la Documentazione](#Aggiungere-la-Documentazione) 
- [Generare la Documentazione](#Generare-la-Documentazione)
###### [§ External Crates](#-External-Crates-1)
___
## **§ Documentazione**

**Uso**: Ai fini di chiarezza del codice
**Sintassi**: `///`
**Tags**: #Documentazione #Cargo
	
- ### Aggiungere la Documentazione
	
```Rust
/// This is an enumeration
enum Enumeration {
	/// Variant1
	Variant1,
	/// Variant2
	Variant2,
	/// Variant3
	Variant3
}

/// This is a structure
struct Structure {
	/// This is the most important field
	field1: u8,
	field2: String,
	field3: bool,
}

/// This is a function
fn function (a: i8, b: i8) -> i8 {
	/// Adds two numbers together
	a + b
}

fn main (){}

```
	
- ### Generare la Documentazione
	
```sh
$ cargo doc --open
```
	
	
---
## **§ External Crates**



---
##### Progressione Suggerita
[Rust Cheat Sheet - Tipi](rust_types_cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini