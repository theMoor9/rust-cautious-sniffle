## **Rust Cheat Sheet - Semplificare la Gestione Dati**
##### **Table of Contents**
###### [§ Closures](#-Closures-1)
- [Basics](#Basics)
###### [§ Map Combinator](#-Map-Combinator-1)
###### [§ Option Combinator Pattern](#-Option-Combinator-Pattern-1)
###### [§ ](#- -1)
	
---
## **§ Closures**
	
- **Descrizione**: Le closure sono Funzioni semplici senza identità atte a piccole modifiche. Per differenziare le closures da le funzioni classiche si usa i pipes `| ... |` come utilizziamo le parentesi tonde `( ... )` . Possono inoltre auto definire i loro parametri della loro firma nella forma breve.
- **Tags**: #Closures 
- **Sintassi**: 
```Rust
// Firma Estesa
let closure_name = | a: Type, b: Type | -> Return_Type { ... } ;
// Firma Breve
let closure_name = | a , b | ... ;
```
	
### Basics
	
- **Caso d'Uso**: Si può evitare grosse parti di sintassi utilizzando la forma breve della closure. E possiamo utilizzarle in maniera strategica in situazioni dinamiche, come potrai vedere nelle sezioni (§) di seguito.
- **Esempi**:
	
```Rust
fn main () {
	// Firma e corpo esteso
	let sub = |a: i64, b: i64| -> i64 { a + b };
	// Firma e corpo breve
	let add = |a,b| a + b;
	let x = add(1,1);
	let y = sub(x,1);

	println!("{}",y)
}
```
- **Output**: `1`
	
	
---
## **§ Map Combinator**

- **Descrizione**: Il Mapping è la pratica di  trasformare qualcosa in un altra. La funzione complementare `.map()` in Rust è lo strumento atto a questo scopo che restituisce un iteratore sui dati trasformati, pronti per essere consumati quando necessario.
- **Caso d'Uso**: La funzione `.map()` serve ad applicare Closures( Funzioni semplici atte a piccole modifiche ) in maniera dinamica quindi su risultati di funzione funzione().map(closure) multivalore quindi inaspettati o collezioni di dati, come per formattare in upcase dati che si generano via via che la funzione primaria genera dati per .map()
- **Tags**: #Map #Combinators
- Esercizio:
	
```Rust
// Funzione di somma con input che resituisce un un i8 o un Messaggio di errore
fn sum (a:i8, b:i8, input: Option<i8>) -> Result<i8, String> {
    match input {
        Some(i) => Ok(a + b + i),
        None => Err(String::from("No input!"))
    }
}
  
/*
Vogliamo aggiungere al risultato di sum() una correzione.
Map è dinamico nel caso in cui sum() restituisca un messaggio di errore
.map() non eseguirà la closure.
*/
fn main () {
    let input = Some(1);
    let correction = 6;
    let sum_corrected = sum(1,1,input).map(|sum_result| sum_result + correction);
  
    match sum_corrected {
        Ok(sum_r) => println!("Result: {}", sum_r),
        Err(e) => println!("Error: {}", e)
    }
}
```
- **Output**: `9`
- `None` **Output**: `No Input!`
	
### Collect
	
- **Descrizione**: Il collettore *collect* trasforma un iteratore in una collezione, cosa che può essere necessaria a seconda di come intendi utilizzare i dati.
- **Sintassi**: `.collect()`
- **Caso d' Uso**: Collect finalizza i dati elaborati, se scopo è semplicemente passare i dati trasformati a un altro passo di elaborazione o consumare i risultati una sola volta, potresti non aver bisogno di usare `collect()`.
- **Esercizio**::
	
```Rust
fn main () {

    let names: Vec<&str> = vec![
        "Kenneth",
        "Philip",
        "Linda",
        "Giulia"
    ];
   /*
    Applico .map() a .iter(), nella sua firma indico `s: &str` in forma implicita e
    uso .collect() per convertire l'iteratore risultante in un Vec<String>,
    raccogliendo tutti i nomi trasformati in maiuscolo in un vettore.
    */
	let uppercase_names: Vec<String> = names.iter()
											.map(|s|-> String {s.to_uppercase()})
		                                    .collect();
    println!("{:?}", uppercase_names);
```
- **Output**: `["KENNETH", "PHILIP", "LINDA", "GIULIA"]`
	
	
---
## **§ Option Combinator Pattern**

- **Descrizione**: Lista dei combinators per i tipi Option (Vedi: **§ Tipi Aggiuntivi | Option** in [Rust Cheat Sheet - Tipi](rust_types_cheatsheet.md)).
- **Tags**: #Option #Combinators #Map #Closures 
- **Esempi**:
	
```Rust
fn main() {
	let opt: Option<i8> = Some(3) ;
	// Restituiscono dati booleani sulla veridicità del nome del combinatore
	let opt_is_some = opt.is_some() ; // Output: true
	let opt_s_none = opt.is_none() ; // Output: false
	
	// Itera i dati solo se la Option è Some() in questo caso Some(3) sarà num
	let opt_mapped = opt.map(|num|num + 6) ; //Output: Some(9)
	
	// Opera con una closure di comparazione che usa il borrowing
	let comparison = 3 ;
	let opt_filtered = opt.filter(|num|num == &comparison) ;
	// num = 3 Output: 3 , num != 3 Output: None
	
	// Controlla se opt possiede dati se falso ne assegna uno Some(9)
	let opt_or_else = opt.or_else(||Some(9)) ; // Output: Some(3);
	
	// Utilizza un valore default se `opt` è None 
	let default_val = 9; 
	let unwrapped = opt.unwrap_or_else(|| default_val);
	// Output: unwrapped == 3 if opt == None unwrapped == 9
}
```
	
> Controllare la libreria standard di Rust  per maggiori funzionalità( Vedi: **§ Standard Library API Docs** in [Rust Cheat Sheet - Elementi base](rust_basics_cheatsheet.md).)
	
---
##### Progressione Suggerita
[Rust Cheat Sheet - ](rust_controls_dynamics_cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini