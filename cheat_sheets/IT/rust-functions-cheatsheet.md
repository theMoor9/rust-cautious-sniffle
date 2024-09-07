# **Rust CheatSheet - Funzioni**
##### **Table of Contents**
###### [§ Signature](#-Signature-1) ✒️
###### [§ Metodi di Restituzione](#-Metodi-di-Restituzione-1) ↩️
- [Propagazione Errore](#Propagazione-Errore)
###### [§ Funzionalità (Metodi)](#-Funzionalità-Metodi-1) 🛠️
- [Implementazione](#Implementazione)
- [Implementazione Autonoma](#Implementazione-Autonoma)
###### [§ Funzioni Generiche](#-Funzioni-Generiche-1) 🌐
- [Sintassi](#Sintassi)
###### [§ Metodi Standard](#-Metodi-Standard-1) 🔧
- [Vettori (`Vec<T>`)](#vettori-vect)
- [Stringhe (`String` e `&str`)](#stringhe-string-e-str)
- [From Into](#From-Into)
	
---
## **§ Signature**
	
- **Descrizione**:  Nella Firma della funzione gli oggetti della funzione sono dichiarati col loro tipo `a: i32` , `b: i32` Il simbolo `->`  serve esclusivamente per indicare il tipo della restituzione.
- **Proprietà**: Owner
- **Sintassi**: `fn`
- **Tags**: #Functions #Ownership 
- **Esempio**:
	
```Rust
fn multiply(a: i32, b: i32, c ...) -> i32 {
	...
}
```
	
--- 
## **§ Metodi di Restituzione** :

- **Definizione**: La restituzione è il risulta to delle operazioni della funzione.
- **Tags**: #Functions 
- **Esempi**:
	
	Con `return`
		
	```Rust
	fn multiply(a: i32, b: i32) -> i32 {
		 return a * b ; 
	}
	// Restituisce a * b esplicitamente
	```
		
	Senza `return`
		
	```Rust
	fn multiply(a: i32, b: i32) -> i32 {
		 a * b 
	}
	// Restituisce implicitamente a * b come ultima espressione valutata
	```
		
**Differenza tra `return` e Ultima Espressione**
	
- **Espressione finale senza punto e virgola**: Se l'ultima espressione nel corpo della funzione non è seguita da un punto e virgola, il valore di questa espressione sarà automaticamente restituito.
	
- **Comando `return`**: Puoi usare `return` per restituire un valore in modo esplicito da qualsiasi punto della funzione. Questo può essere utile per restituire valori condizionalmente.
	
### Propagazione Errore
	
- **Descrizione**: In Rust, è possibile propagare gli errori di un `Result` da una funzione a un'altra, delegando la gestione dell'errore alla funzione "madre" che ha chiamato la funzione figlia.
- **Uso**: La propagazione degli errori è utile per simulare una meccanica simile al _raise_ presente in altri linguaggi, consentendo di far risalire l'errore alla funzione chiamante. Questo evita di gestire gli errori a ogni livello intermedio, semplificando il codice.
- **Sintassi**: `?`
- **Tags**: #Functions #Advanced #Result #Error 
- **Esempi**:
	
```Rust
fn son() -> Result<(), Error> {
    // Calcoli che potrebbero generare errori
}

fn mother() -> Result<(), Error> {
    son()?; 
    // Se la chiamata a `son` genera un errore, 
    // questo viene propagato e gestito dalla funzione madre
    Ok(()) // La funzione madre deve comunque restituire un `Result`
}

```
	
	
---
## **§ Funzionalità (Metodi)**
	
### Implementazione
- **Uso**: Implementazione atta a definire funzioni di proprietà  (*Funzionalità* o *Metodi) per i tipi complessi `struct` o `enum` tramite il sistema di integrazione *implement*.
- **Tags**: #Structs #Impl
- **Sintassi**: `impl`
- **Esempio**:
	
```Rust
// Inizializzazione struct
struct Distance {
	meters: f32,
}

// Funzione pertinente alla struct
fn show_distance (marathon: Distance) {
	println!("The distance is {}", marathon.meters);
}

fn main () {
	
	let firenze_cup = Distance{
		meters: 9000.0,
	};
	
	show_distance(firenze_cup);
}
```
	
Manipolazione delle `fn` tramite `impl` in metodi:
	
```Rust
struct Distance {
	meters: f32,
}

// Implementazione del metodo con riferimento al nome della struct Distance
impl Distance {
		
	// La funzione diventa un metodo
	fn show_distance (marathon: Distance) {
		println!("The distance is {}", marathon.meters);
	}
}

fn main () {
	let firenze_cup = Distance{ meters:9000.0 };
	// Chiamata del Medodo indiretta!
	Distance::show_distance( firenze_cup )
	// Restituisce a monitor "9000.0"
}
```
	
```Rust
struct Distance {
	meters: f32,
}
	
impl Distance {
		
	/* 
	La funzione diventa un metodo con riferimento a chi la chiama &,
	Può così essere chiamata più volte!
	*/
	fn show_distance (&self) {
		println!("The distance is {}", self.meters);
	}
	
}

fn main () {
	let firenze_cup = Distance{ meters:9000.0 };
	// Chiamata del Metodo diretta!
	firenze_cup.show_distance();
	// Restituisce a monitor "9000.0"
}
```
	
### Implementazione Autonoma
- **Descrizione**: Quando i valori della `struct` non sono inizializzati `impl` può autodefinire i valori al momento della chiamata del metodo grazie alla ***Restituzione*** `-> Self`
- **Tags**: #Structs 
- **Sintassi**: `-> Self`
- **Esempio**:

```Rust
struct Distance {
	meters: f32,
}

impl Distance {
	fn set_distance () -> Self {
		Self { meters:9000.0 }
	}
	fn input_distance (input: f32) -> Self {
		Self { meters: input }
	}
	fn show_distance (&self) {
		println!("The distance is {}", self.meters);
	}
}
	
fn main () {
	firenze_cup.set_distance()
	firenze_cup.show_distance()
	// Restituisce a monitor "9000.0"
	firenze_cup.input_distance(999.9)
	firenze_cup.show_distance()
	// Restituisce a monitor "999.9"
}
```
	
	
---
## **§ Metodi Standard**
	
### Vettori (`Vec<T>`)
	
**Tags**: #Vectors 
	
- **.push()**: Aggiunge un elemento alla fine del vettore.
- **.pop()**: Rimuove e restituisce l'ultimo elemento del vettore.
- **.len()**: Restituisce il numero di elementi nel vettore.
- **.is_empty()**: Controlla se il vettore è vuoto.
- **.insert(index, element)**: Inserisce un elemento in una posizione specificata, spostando gli altri elementi.
- **.remove(index)**: Rimuove l'elemento alla posizione specificata e lo restituisce.
- **.clear()**: Rimuove tutti gli elementi dal vettore.
- **.contains(&element)**: Controlla se il vettore contiene un elemento specifico.
- **.append(&mut Vec<_T>)**: Aggiunge tutti gli elementi di un altro vettore al vettore corrente.
- **.split_off(mid)**: Divide il vettore in due a una posizione specificata e restituisce la seconda metà.
- **.capacity()**: Restituisce la capacità totale del vettore.
- **.reserve(additional)**: Riserva spazio per almeno altri `additional` elementi.
- **.shrink_to_fit()**: Riduce la capacità del vettore per adattarla alla lunghezza attuale.
- **.retain(f)**: Mantiene solo gli elementi che soddisfano il predicato `f`.
- .as_slice(): Converte in slice per essere iterato come slice.
	
### Stringhe (`String` e `&str`)
	
**Tags**: #Strings 
	
- **.to_string()**: Converte una slice di stringa in una stringa posseduta.
- **.to_owned()**: Crea una copia posseduta della stringa.
- **.len()**: Restituisce la lunghezza della stringa in byte.
- **.is_empty()**: Controlla se la stringa è vuota.
- **.push_str(&str)**: Aggiunge una slice di stringa alla fine di una stringa `String`.
- **.push(char)**: Aggiunge un singolo carattere alla fine di una stringa `String`.
- **.clear()**: Rimuove tutti i caratteri dalla stringa.
- **.contains(&str)**: Controlla se la stringa contiene una sottostringa specifica.
- **.replace(&str, &str)**: Restituisce una nuova stringa con tutte le occorrenze di una sottostringa specifica sostituite con un'altra sottostringa.
- **.trim()**: Restituisce una nuova stringa con gli spazi bianchi iniziali e finali rimossi.
- **.split_whitespace()**: Restituisce un iteratore sugli elementi della stringa separati dagli spazi bianchi.
- **.as_str()**: Restituisce una slice `&str` di una stringa `String`.
- **.capacity()**: Restituisce la capacità totale della stringa.
- .as_slice(): Converte in slice per essere iterato come slice.
	
### From Into
	
**Descrizione**: Sono metodi di *Traits* che permettono di convertire un tipo in un altro
**Uso**: Si utilizza ad esempio nella generazione dello String type con `String::from("slice")`. E' spesso utile implementare `from`  o con `.into()`  per gli errori rendendoli dinamici e convertibili tra le varie tipologie
**Tags**: #Traits 
**Esempio**:

```Rust
enum Status {
	Online,
	Offline,
	NoResponse(u8),
}

/*
* NON necessario in quanto parte della libreria standard di rust.
trait From {
	fn from(code: u8) -> Self;
}
*/

impl From<u8> for Status {
	fn from(code: u8) -> Self {
		match code {
			0 => Status::Offline,
			1 => Status::Online,
			c => Status::NoResponse(c),
		}
	}
}
fn status_code_gen(n: u8) -> u8 {
	match n {
		1 => 9,
		2 => 3,
		_ => 0
	}
}

let into_status: Status = status_code_gen(1).into(); 
let from_trait_status = Status::from(status_code_gen(2));

println!("{:?}",into_status);
println!("{:?}",from_trait_status);
```
- **Output**:
```sh
NoResponse(9)
NoResponse(3)
```
	
	
---
##### Progressione Suggerita
[Rust CheatSheet - Cicli](rust-loops-cheatsheet.md)
[Rust CheatSheet - Dinamiche del codice](rust-control-dynamics-cheatsheet.md)
[Rust CheatSheet - Semplificare la Gestione Dati](rust-data-management-cheatsheet.md)
	
---
	
**Author:** Kenneth Boldrini