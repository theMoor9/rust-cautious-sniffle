# **Rust Cheat Sheet - Cicli**
	
---
##### **Table of Contents**
###### [§ Ciclo Infinito ∞](#[-Ciclo-infinito--1)
###### [§ Cicli Condizionale - while](#-Cicli-Condizionale-1)
###### [§ Ciclo Iterativo - for](#-Ciclo-Iterativo-1)
###### [§ Interruzione Cicli](#-Interruzione-Cicli-1)
	
---
## **§ Ciclo infinito ∞**
	
- **Descrizione**: Ciclo infinito
- **Sintassi**: `loop` 
- **Tags**: #Loops 
- **Esempio**:
	
```Rust
loop {
	print!("I'am cycling in a long text line, ");
}
```
	
	
---
## **§ Cicli Condizionale**
	
##### WHILE
	
- **Descrizione**: Cicla in base alle condizioni date
- **Sintassi**: `while`
- **Tags**: #Loops 
- **Esempio**:
	
```Rust
while true {
	print!("I'am cycling in a long text line, ");
}
```
	
	
---
## **§ Ciclo Iterativo**
	
##### FOR
	
- **Descrizione**: Cicla ispezionando in maniera sequenziale gli elementi in esame
- **Sintassi**: `for`
- **Tags**: #Loops #Vectors #Arrays #Tuples #Enums
- **Esempio**:
	
	*Range*:    `n..n-1`
	```Rust
	for i in 1..5 { 
		println!("{}", i) ; // Stampa 1, 2, 3, 4
	} 
	```
	
	
	*Range inclusivo*:    `n...=n`
	```Rust
	for i in 1..=5 { 
		println!("{}", i) ; // Stampa 1, 2, 3, 4, 5 
	}
	```
	
	
	*Range con Passo*:    `.step_by()`
	```Rust
	for i in (1..10).step_by(2) { 
		println!("{}", i) ; // Stampa 1, 3, 5, 7, 9 
	}
	```
	
	
	*Array* :    `.iter()` 
	```Rust
	let array = ["apple", "banana", "cherry"] ; 
	for fruit in fruits.iter() { 
		println!("{}", fruit) ; // Stampa apple, banana, cherry
	}
	```
	
	
	*Enumerazioni* :    `.enumerate()`
	```Rust
	let vec = vec!["a", "b", "c"] ; 
	for (index, val) in vec.iter().enumerate() { 
		println!("Index: {}, Value: {}", index, val) ; 
		// Stampa Index: 0, Value: a, ecc. 
	}
	```
	
	
	*Tuple* :    Destrutturazione `for (key,value)` 
	```Rust
	let tuples = vec![(1, "a"), (2, "b"), (3, "c")] ; 
	for (num, letter) in tuples { 
		println!("Number: {}, Letter: {}", num, letter) ; 
		// Stampa Number: 1, Letter: a, ecc.
	}
	```
	
	
	*Collezioni* :    
	- *Iterazione per Valore* (`.into_iter()`): Consuma la collezione e sposta gli elementi  
		```Rust
		let vec = vec![1, 2, 3, 4, 5] ; 
		for val in vec.into_iter() { 
			println!("{}", val) ; // Consuma e stampa i valori 
		} 
		// Stesso risultato ma consumazione implicita 
		let vec = vec![1, 2, 3, 4, 5] ; 
		for val in vec { 
			println!("{}", val) ; // Consuma e stampa i valori 
		}
		// Dopo questo, 'vec' non è più utilizzabile
		```
	 - *Iterazione per Riferimento Mutable* (`.iter_mut()`): Permette di modificare gli elementi durante l'iterazione.
		```Rust
		let mut vec = vec![1, 2, 3, 4, 5] ; 
		for val in vec.iter_mut() { 
			*val *= 2 ; // Modifica i valori dell'array
		} 
		println!("{:?}", vec) ; // Stampa [2, 4, 6, 8, 10]
		```
	
	
	*HashMap*:    `for (key, value) in &map`
	```Rust
	use std::collections::HashMap ;
	
	let mut map = HashMap::new() ; 
	map.insert("a", 1) ; map.insert("b", 2) ; 
	for (key, value) in &map { 
		println!("Key: {}, Value: {}", key, value) ; 
		// Stampa Key: a, Value: 1, ecc.
	}
	```
	
	
	*VecDeque*:    `for val in &deque`
	```Rust
	use std::collections::VecDeque ;

	let mut deque = VecDeque::new() ; 
	deque.push_back(1) ; 
	deque.push_back(2) ; 
	deque.push_back(3) ; 
	for val in &deque { 
		println!("{}", val) ; // Stampa 1, 2, 3
	}
	```
	
	
	*Iterazione avanzata con funzione di mappatura*:
	```Rust
	let vec = vec![1, 2, 3, 4, 5] ; 
	
	let squared: Vec<_> = vec.iter().map(|x| x * x).collect() ; 
	for val in squared { 
		println!("{}", val) ; // Stampa 1, 4, 9, 16, 25 
	}
	```
	
	
---
## **§ Interruzione Cicli**
	
- `continue`: Salta l'iterazione corrente e passa alla successiva.
	
- `return`: Esce dalla funzione in cui il ciclo è contenuto.
	
- `labelled break` e `labelled continue`: Per gestire cicli annidati.
```Rust
outer: for i in 0..5 { 		
	for j in 0..5 { 
		if i == 2 && j == 2 { 
			break 'outer ; // Uscita dal ciclo esterno 
			
			// continue 'outer ;
			// passa alla prossima iterazione del ciclo esterno
		} 
	println!("i: {}, j: {}", i, j) ; 
}
```
	
	
- `break` con valore: Per restituire un valore da un ciclo `loop`.
```Rust
let result = loop { 
	counter += 1 ; 
	
	if counter == 10 { 
		break counter * 2 ; // Restituisce il valore counter * 2 
	}
} ;
println!("Il risultato è {}", result) ; // Stampa "Il risultato è 20"
```
	
	
- Uso di `Option` e `Result` per gestire condizioni di uscita specifiche.
	
	Con `Option`
	```Rust
	fn main() {
		let vec = vec![1, 2, 3, 4, 5] ;
		
		for &val in vec.iter() {         
			if val == 3 {             
				break Some(val) ; // Restituisce Some(3)         
			}     
		} 
	}`
	```
	
	
	Con `Result`
	```Rust
	fn main() -> Result<(), &'static str> {     
		let vec = vec![1, 2, 3, 4, 5] ;      
		
		for &val in vec.iter() {         
			if val == 3 {             
				return Err("Errore: trovato il valore 3") ; 
				// Restituisce un errore         
			}     
		}      		
		Ok(()) 
	}
	```
	
	
---
##### Suggested Progression
[Rust Cheat Sheet - Macro](./rust_cheat_sheet_macros.md)
	
---
	
**Author:** Kenneth Boldrini
