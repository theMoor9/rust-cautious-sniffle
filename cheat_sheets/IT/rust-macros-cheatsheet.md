# **Rust CheatSheet - Macro**
##### **Table of Contents**
###### [§ println!](#-println-1)
- [Tipologie di Placeholders](#Tipologie-di-Placeholders)
###### [§ vec!](#-vec-1)
###### [§ stringify!](#-stringify-1)
###### [§ dbg!](#-dbg-1)
###### [§ Derive](#-Derive-1)
- [Debug](#Debug)
- [Clone](#Clone)
- [Quando Usare Copy](#Quando-Usare-Copy)
- [Riassunto](#Riassunto)
###### [§ Config](#-Config-1)
- [Target](#Target)
- [Test](#Test)
	
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
	
- **Descrizione**: Macro che consente la generazione dei vettori di tipo univoco, a cui si applica tutte le proprietà delle *struct* `Vec` (Vedi: **§ Tipi Complessi | Struct -  Struct Vec** in [Rust Cheat Sheet - Tipi](rust-types-cheatsheet.md)).
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
## **§ stringify!**
	
- **Descrizione**: Converte il nome di una variabile in una stringa.
- **Tags**: #Strings #Macros #Types 
- **Esempio**:
	
```Rust
let kenneth = "Kenneth";
let name = stringify!(kenneth);
println!("{}",name);
```
- **Output**: `kenneth`
	
	
---
## **§ dbg!**
	
- **Descrizione**: Mostra nel terminale l'argomento ai fini del debug.
- **Tags**: #Debug #Macros  
- **Esempio**:
	
```Rust
let kenneth = "Kenneth";
let name = stringify!(kenneth);
dbg!(name);
```
- **Output**: `name = kenneth`
	
	



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
- **Uso**: Permette di non violare la condizione di *ownership* di un oggetto .
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
	
- **Descrizione**: La specificazione `Copy` rispetto a `Clone` indica che è permessa la copia del clone automatica e quindi implicita senza bisogno di chiamare `.clone()`. Con `#[derive(Clone, Copy)]`, la clonazione è sia esplicita che implicita.
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
## § Config
	
**Descrizione**: Il macro `cfg` in Rust è utilizzato per configurare la compilazione condizionale del codice a seconda di specifiche flag o opzioni di configurazione.
**Uso**: Particolarmente utile per includere o escludere parti di codice basandosi su piattaforme target, funzionalità opzionali, o altre condizioni personalizzate definite dall'utente.
**Tags**: #Config
	
### Target
	
- **Descrizione**: Compilare codice in base all' OS
- **Tags**: #Os 
- **Esempio**:
	
```Rust
// Compila questo codice solo se il target è Windows. 
#[cfg(target_os = "windows")] 
fn are_you_on_windows() { println!("Siamo su Windows!"); } 

// Compila questo codice solo se il target è Linux. 
#[cfg(target_os = "linux")] 
fn are_you_on_linux() { println!("Siamo su Linux!"); }
```
	
### Test 
	
- **Descrizione**: Con `test`, aggiungiamo implementazioni di compilazione per il debug come `mod`  per scopi di debug
- **Uso**: Con l incrementare della complessità dei programmi è buona pratica implementare test per la robustezza (i tende a eseguire il codice prendendo in considerazione i casi estremi del test).
- **Tags**: #Test #Modules #Functions 
- **Esempio**:

```Rust
#[cfg(test)]
mod connection {
	#[test]
	fn init (_) {_} // si applica test su questa funzione
	
	fn abort (_) {_}
	fn check (_) {_}
}
```
	
##### assert! 
	
-  **Uso**: Verifica se il primo argomento della firma è `true`
-  **Sintassi**: `assert!(boolean_var, "Debug return message");
- **Esempio**:
	
	```Rust
	#[cfg(test)]
	mod testing {
		
		#[test]
		fn is_true {
			let boolean: bool = true
			assert!(boolean,"Is not true!");
		}
		
	}
	```
	
##### assert_eq!
	
-  **Uso**: Verifica se il primo argomento della firma è *uguale* al secondo
-  **Sintassi**: `assert_eq!(result, expected, "Debug return message");
- **Esempio**:
	```Rust	
	mod testing {
		
		#[test]
		fn equality {
			let me = "Sapiens"
			let you = "Sapiens"
			assert_eq!(me,you, "They are NOT the species!");
		}
		
		
		
	}
	```
	
##### assert_ne!
	
-  **Uso**: Verifica se il primo argomento della firma è *diverso* al secondo
-  **Sintassi**: `assert_ne!(result, not_expected, "Debug return message");
- **Esempio**:
	
```Rust
	mod testing {
		
		#[test]
		fn equality {
			let me = "Sapiens"
			let my_cat = "Felidae"
			assert_ne!(me,you, "They ARE the species!");
		}
		
	}
```
	
	
---
##### Suggested Progression
[Rust CheatSheet - Dinamiche del codice](rust-controls-dynamics-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini
