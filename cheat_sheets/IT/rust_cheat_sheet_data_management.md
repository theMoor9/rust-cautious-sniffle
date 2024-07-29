## **Rust Cheat Sheet - Semplificare la Gestione Dati**
##### **Table of Contents**
###### [§ Closures](#-Closures-1)
###### [§ Map Combinator](#-Map-Combinator-1)
###### [§ ](#- -1)
###### [§ ](#- -1)
	
---
## § Closures
	
### Basics
- **Descrizione**: Funzione anonima creabile all interno del codice definibile solo all interno di una funzione. Da considerare come una funzione annidata. 
- **Sintassi**: 
```Rust
// Estesa
let closure_name = | a: Type, b: Type | -> Return_Type { ... } ;
// Breve
let closure_name = | a , b | ... ;
```
- **Caso d'Uso**: Si può evitare grosse parti di sintassi utilizzando la forma breve della closure.
- **Tags**: #Closures 
- **Esempi**:
	
```Rust
fn main () {
	let sub = |a: i64, b: i64| -> i64 { a + b };
	let add = |a,b| a + b;
	let x = add(1,1);
	let y = sub(x,1);

	println!("{}",y)
}
```
- **Output**: `1`
	
	
---
## § Map Combinator

IL Mapping è trasformare qualcosa in un altra