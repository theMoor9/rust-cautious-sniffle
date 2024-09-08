# **Rust CheatSheet - Cargo**
##### **Table of Contents**
###### [§ Documentazione](#-Documentazione-1) 📃
- [Aggiungere la Documentazione](#Aggiungere-la-Documentazione)
- [Generare la Documentazione](#Generare-la-Documentazione)
###### [§ Installare Crate](#-Installare-Crate-1) 📦
	
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
## **§ Installare Crate**

**Uso**: Installazione crate tramite cargo garantendo ultima versione disponibile.
**Tags**: #Cargo #ExternalLibraries #Crates 
**Esempio**:
	
```sh
cargo install crate_name
```
	
	
---

>E' consigliato usare i `Tags` in relazione a gli altri Cheatsheets per un quadro sull'argomento più esaustivo.
##### Progressione Suggerita
[Rust CheatSheet - Tipi](rust-types-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini