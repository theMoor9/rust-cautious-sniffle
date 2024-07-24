# **Rust Cheat Sheet - Macro**
##### **Table of Contents**
###### [§ println!](#-println-1)
- [Tipologie di Placeholders](#Tipologie-di-Placeholders)
###### [§ vec!](#-vec-1)
###### [§ Derive](#-Derive)
- [Debug](#Debug)
- [Clone](#Clone)
- [Quando Usare Copy](#Quando-Usare-Copy)
- [Riassunto](#Riassunto)

	
---
## **§ println!**
	
- **Descrizione**: Oltre il classico uso, `println!` ha bisogno elementi *`token`* da usare nei placeholders `{...}` come argomenti oltre la stringa per stampare variabili
- **Tags**: #Macros 
- **Esempi**:
	
```Rust
let token = value
println!("string {}", token)
// Sostituisce {} con il valore di token_value
```
	
	
### Tipologie di Placeholders
	
- **Descrizione**: E' possibile scegliere tra diverse tipologie di placeholders ai fini della formattazione
- **Esempi**:
	
*Valore end-user display*
```Rust
println!("Il valore di x è: {}. capito? {}", x, b) ;
// Sostituisce {} con i valori di x e b rispettivamente

println!("Il valore di x è: {x}. capito? {b}") ;
// Con Rust 1.58, sostituisce {x} e {b} con i valori di x e b rispettivamente
```
	
	
>I due punti `:` vengono utilizzati per introdurre specifiche 
>di formattazione all'interno di una stringa di formattazione, quindi segue:
	
	
*Valore debug non formattato*
```Rust
println!("L'array è: {:?}", arr) ;
// Sostituisce {:?} con il valore debug di arr

println!("L'array è: {arr:?}") ; 
// Con Rust 1.58, sostituisce {arr:?} con il valore debug di arr
```
	
	
*Valore debug formattato*
```Rust
println!("L'array formattato è: {:#?}", arr) ;
// Sostituisce {:#?} con il valore debug formattato di arr
```
	
	
*Valore ordinato per posizione*
```Rust
println!("{1} viene dopo {0}", a, b) ;
//Sostituisce {1} con il secondo argomento (b) e {0} con il primo argomento (a)
```
	
	
*Valore decimale*
```Rust
println!("Pi è approssimativamente {:.2}", pi) ;
// Sostituisce {:.2} con il valore di pi formattato a 2 decimali
```
	 
	
*Formattazione spaziale di accostamento*
```Rust
println!("Il valore di x è: {:5}", x) ; 
// Stampa "Il valore di x è:     42"

println!("Allineamento a destra: {:>5}", x) ; 
// Stampa "Allineamento a destra: 42" 

println!("Allineamento a sinistra: {:<5}", x) ; 
// Stampa "Allineamento a sinistra: 42     "
```
	
	
---
## **§ vec!**
	
- **Descrizione**: Macro che consente la generazione dei vettori di tipo univoco, a cui si applica tutte le proprietà delle *struct* `Vec` (Vedi [[rust_cheat_sheet_types]]).
- **Tags**: #Vectors #Macros #Types 
- **Esempi**:
	
```Rust
// Inizializzazione
let week = vec!["mon","tue","wed","thu","fri","sat","sun"] ;

// Selezione
// let vector_positions = vec![0,1,2,3,4,5,6,7]
let saturday = week[5] ;
```
	
Questa macro permette in oltre di generare vettori che contengono tupi complessi
	
```Rust
struct Volume{
	value: u8,
}

fn main() {
	// Crea un vettore fatto di struct differenti
	let settings = vec![
		Volume{ value: 00 },
		Volume{ value: 10 },
		Volume{ value: 20 },
		Volume{ value: 30 },
	];
	
	// Per la stampa della struttura in questione occore che il token sia `{:?}`
	for v in settings {
		println!("The struct setting is increasing to: {:?}", v);
	}
	
	/*
	Output:
	The struct setting is increasing to: Volume { value: 00 }
	The struct setting is increasing to: Volume { value: 10 }
	The struct setting is increasing to: Volume { value: 20 }
	The struct setting is increasing to: Volume { value: 30 }
	*/
}
```
	
---
## **§ Derive**
	
- **Descrizione**: Con `derive`, aggiungiamo implementazioni automatiche di determinati tratti ai tipi complessi come `enum` e `struct` per scopi di debug o convenienza.
- **Tags**: #Macros 
	
### Debug
	
- **Descrizione**: La macro `#[derive(Debug)]` viene utilizzata per assegnare automaticamente l'implementazione del tratto `Debug` a una struttura o un'enumerazione. Questo permette di formattare il tipo in modo leggibile, rendendolo utile per il debug.
- **Tags**: #Structs #Enums 
- **Esempio**:
	

```Rust
#[derive(Debug)]
enum EyeColor {
	Green,
	Blue,
	Brown,
}

#[derive(Debug)]
struct Person{
	name: String,
	age: u8,
	eyes: EyeColor,
}

fn main () {
	let me = Person{
		name: String::from("Kenneth"),
		age: 27,
		eyes: EyeColor::Brown,
	};
	
	println!("{:#?}", me.eyes);
	//Debug Output: Brown

	println!("{:#?}", me);
	/* 
	Debug Output: 
	Person { 
	   name: "Kenneth", 
	   age: 27, 
	   eyes: Brown, 
	   }
	*/
}
```
	

In pratica, `#[derive(Debug)]` facilita l'estrapolazione e la rappresentazione del codice per il debug, consentendo l'uso del codice in contesti che altrimenti non lo permetterebbero.
	
### Clone
	
- **Descrizione**: La macro `#[derive(Clone)]` permette di creare copie della struttura complessa. `Clone` indica che è permessa la clonazione esplicita chiamando `.clone()`.
- **Caso d'Uso**: Permette di non violare la condizione di *ownership* di un oggetto .
- **Tags**: #Structs #Enums #Ownership 
- **Esempio**:
	
```Rust
#[derive(Debug)]
enum EyeColor {
	Green,
	Blue,
	Brown,
}

#[derive(Clone,Debug)]
struct Person{
	name: String,
	age: u8,
	eyes: EyeColor,
}

fn print_age (worker: Person) {
	println!("{:#?}", worker.age);
}
fn print_eye_color (worker: Person) {
	println!("{:#?}", worker.eyes);
}

fn main () {
	let me = Person{
		name: String::from("Kenneth"),
		age: 27,
		eyes: EyeColor::Brown,
	};
	print_age(me.clone())
	print_eye_color(me.clone())
	//Debug Output: 27
	//Debug Output: Brown
}

```
	
>`Clone`: Permette la clonazione esplicita chiamando `.clone()`.
	
### Copy
	
- **Descrizione**: La specificazione `Copy` rispetto a `Clone` indica che è permessa la copia del clone automatica e quindi implicita senza bisogno di chiamare `.clone()`. Con `#[derive(Clone, Copy)]`, la clonazione è sia esplicita che implicita:
- **Tags**: #Structs #Enums 
- **Esempio**:
	
```Rust
#[derive(Debug)]
enum EyeColor {
	Green,
	Blue,
	Brown,
}

#[derive(Clone,Copy,Debug)]
struct Person{
	name: String,
	age: u8,
	eyes: EyeColor,
}

fn print_age (worker: Person) {
	println!("{:#?}", worker.age);
}
fn print_eye_color (worker: Person) {
	println!("{:#?}", worker.eyes);
}

fn main () {
	let me = Person{
		name: String::from("Kenneth"),
		age: 27,
		eyes: EyeColor::Brown,
	};
	print_age(me)
	print_eye_color(me)
	//Debug Output: 27
	//Debug Output: Brown
}
```
	

>`Copy`: Permette la copia automatica implicita senza bisogno di chiamare `.clone()`. Questo è utile per tipi semplici e di dimensioni fisse che non gestiscono risorse dinamiche.
	
 **Quando Usare Copy**
	
- Usa `Copy` quando il tuo tipo è semplice e può essere copiato in modo sicuro e veloce, come tipi scalari (`i32`, `f64`, ecc.) o tipi composti di tali tipi (strutture con solo tipi scalari).
- Non usare `Copy` su tipi che gestiscono risorse allocate dinamicamente (come `String` o `Vec<T>`), poiché ciò potrebbe portare a problemi di gestione della memoria.
	
#### Riassunto
	
Usando in maniera standard `#[derive(Debug, Clone, Copy)]`, puoi ampliare la funzionalità dei tipi complessi beneficiando di:
- Clonazione esplicita con `.clone()`
- Copia implicita per tipi che implementano `Copy`
- Formattazione per il debug con `Debug`

**Applicazione**:

- `.clone()`: Utilizzabile per qualsiasi tipo che implementa `Clone`, inclusi tipi complessi come `String` e `Vec`.
- `Copy`: Limitato a tipi che hanno una dimensione fissa e non richiedono gestione della memoria dinamica.
	
	
---
##### Suggested Progression
[Rust Cheat Sheet - Dinamiche del codice](rust_cheat_sheet_controls_dynamics.md)
	
---
	
**Author:** Kenneth Boldrini
