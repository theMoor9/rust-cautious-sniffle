# **Rust Cheat Sheet - Tipi**
	
#Types #Integers #Float #Bool #Char
	
---
##### **Table of Contents**
###### [§ Tipi Scalari](#-Tipi-Scalari-1)
- [Numeri Interi](#Numeri-Interi)
- [Numeri Floating-Point](#Numeri-Floating-Point)
- [Booleani](#Booleani)
- [Caratteri](#Caratteri)
###### [§ Tipi Composti](#-Tipi-Composti-1)
- [Tuples](#Tuples)
- [Arrays](#Arrays)
###### [§ Tipi Complessi](#-Tipi-Complessi-1)
- [Struct](#Struct)
- [Enumerations](#Enumerations)
###### [§ Tipi Aggiuntivi](#-Tipi-Aggiuntivi-1)

###### [§ Annotazioni Esplicite del Tipo](#-Annotazioni-Esplicite-del-Tipo)
- [Specifiche](#Specifiche)
- [Generiche](#Generiche)
	
---
## **§ Tipi Scalari** 
	
I tipi scalari in Rust rappresentano i valori più semplici, quelli che non possono essere suddivisi ulteriormente.
	
### Numeri Interi
    
- **Interi Polari**: (`i` : Integers) Possono assumere forma negativa e positiva.
	
	- **Tipi**: ```i8```, ```i16```, ```i32```, ```i64```, ```i128```, ```isize```
	- **Caso d'Uso**: Immagazzinare Temperatura, e variabili che possono assumere valori negativi.
	- **Esempio**:
	
		```Rust
		let age: i32 = 30; 
		let temperature: i16 = -5;
		```           

- **Interi Non Polari**: (`u`: Unsigned) Sono solo numeri positivi.
	
	- **Tipi**: ```u8```, ```u16```, ```u32```, ```u64```, ```u128```, ```usize```
	- **Caso d'Uso**: Età, Indicizzazione array, etc.
	- **Esempio**:
	
		```
		let item_count: u32 = 50; 
		let index: usize = 3;
		```
            
### Numeri Floating-Point
    
- **Definizione**: Usato per i numeri decimali.
- **Tipi**: ```f32``` (Precisione singola), ```f64``` (Precisione doppia)
- **Caso d'Uso**: Calcoli scientifici e di precisione.
- **Esempio**:
	
	```
	let pi: f32 = 3.14; 
	let e: f64 = 2.718281828459045;
	```
        
### Booleani
    
- **Definizione**: Rappresentano vero o falso.
- **Tipo**: ```bool```
- **Caso d'Uso**: Controlli condizionale e avvisi flag.
- **Esempio**:
	
	```
	let is_active: bool = true; 
	let has_error: bool = false;
	```
	
#### Caratteri:
    
- **Definizione**: Rappresenta un codice univoco, racchiuso tra apici.
- **Tipo**: ```char```
- **Caso d'Uso**: Immagazzinare caratteri e simboli singoli.
- **Esempio**:
	
	```
	let letter: char = 'A'; 
	let emoji: char = '😊';
	```
	
	
---
## **§ Tipi composti**
	
I tipi composti raggruppano più variabili sotto un unico tipo
	 
### Tuples
	
- **Definizione**: Può raggruppare molteplici valori con differenti tipi
- **Caso d'Uso**: Ritornare valori multipli da una funzione.
- **Esempio**:
	
```Rust
// Unpacking della tupla in variabili definite
let (first, second, third) = (1,2,3) ; 

// Packing delle variabili in una tupla
let numbers = (first, second, third) ; 

// Accesso tramite posizione dell'elemento
println!("The First Number Is {}", numbers.0) ; // Selezione posizionale .1
// Visualizza a monitor "The First Number Is 1"

println!("The Second Number Is {}?", numbers.1) ; // Selezione posizionale .2
// Visualizza a monitor "The Second Number Is 2?"

println!("The Third Number Is {}!", numbers.2) ; // Selezione posizionale .3
// Visualizza a monitor "The Third Number Is 3!"
```
	
> Si suggerisce di usare le `struct` quando si ha più di due o tre ***campi***
        
	
### Arrays
    
- **Definizione**: Una collezione di elementi prefissata.
- **Caso d'Uso**: Immagazzinare dei dati fissi come i giorni della settimana.
- **Esempio**:

```Rust
let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
let first_day = days[0]; 
println!("First day: {}", first_day);
```

- **Output**: `First day: Mon`
	
#Tuples #Arrays
	
---
## **§ Tipi Complessi**
	
I tipi complessi in Rust rappresentano i valori composti, che  possono essere suddivisi ulteriormente per ottenere una varietà ampia di valori su cui è possibile applicare metodi e attribuire proprietà. 
	
### Struct
	
- **Definizione**: E' un tipo complesso che ha scopo di definire un oggetto avente le "proprietà", chiamate ***campi***, della struttura. I campi si definiscono con loro tipo.
- **Tipo**: `struct`
- **Caso d'Uso**: Definizione di oggetti con proprietà fisiche e metafisiche
- **Esempio**:
	
```Rust
struct ShippingBox{
	width: u32, 
	height: u32,
	depth: u32,
}

/* 
Si u32 invece di i32 perchè chiaramente nessuna scatola 
può essere di misura negativa
*/

let my_box = ShippingBox {
	width: 9,
	height: 6,
	depth: 3,
} ; 
```
	
Si chiama in causa un ***campo*** della struct utilizzando il punto `.`
	
```Rust
let my_box_height = my_box.height ;
let my_box_depth = my_box.depth ;
let my_box_width = my_box.width ;

let my_box_volume = my_box.width * my_box.depth * my_box.height ;
```
	
>Le proprietà di solidità di Rust impongono che alla creazione `let` di una `struct` si debba specificare tutti i ***campi*** di essa per evitare errori
	
- ##### Struct String 
	
	- **Definizione**: E' una collezione mutabile e dinamicamente allocata di caratteri UTF-8.
	- **Tipo**: `String`
	- **Caso d'Uso**: Usa `String` per stringhe mutabili e possedute, e `&str` per riferimenti immutabili a stringhe esistenti.
	- **Esempio**:
	
	```Rust
	// Creazione di una stringa 
	let mut s = String::from("Hello"); 
	
	// Modifica della stringa 
	s.push_str(", world!"); 
	s.push('!'); 
	
	// Concatenazione delle stringhe 
	let s2 = String::from(" How are you?"); 
	let s3 = s + &s2; // s viene spostato qui e non può più essere usato 
	
	// Iterazione sui caratteri della stringa 
	for c in s3.chars() { 
		println!("{}", c); 
		} 
		
	// Accesso ai sottostringhe 
	let hello = &s3[0..5]; let world = &s3[7..]; 
	println!("Substring 1: {}", hello);
	// Output: Hello 
	println!("Substring 2: {}", world); 
	// Output: world! How are you? 
	
	// Conversione tra String e &str 
	let slice: &str = &s3; 
	let s4 = slice.to_string(); 
	println!("Original String: {}", s3); 
	println!("Converted String: {}", s4); 
	
	// Altre funzioni utili 
	let len = s4.len(); 
	let is_empty = s4.is_empty(); 
	println!("Length: {}", len); 
	// Output: 22 
	println!("Is Empty: {}", is_empty); 
	// Output: false 
	let mut s5 = String::from("Temporary"); 
	s5.clear(); 
	println!("Cleared String: {}", s5); 
	// Output: (empty string)
	```
	
>	Con lo `Struct String`, il valore nella memoria, quando viene modificato usa lo stesso spazio piuttosto creare un altro valore nella memoria. Agendo sul valore della variabile come una proprietà dell'oggetto "classe"

- ##### Struct Vec

	- **Definizione**: Una collezione di dati dal tipo univoco
	- Tipo: `Vec<>`
	- Caso d'uso: Collezionare dati dello stesso tipo come i giorni della settimana
	- Esempio:
	```Rust
	let slice: Vec<i32> = Vec::new() ;
	println!("{:?}", slice); // Output: []
	
	let ten_positions: Vec<&str> = Vec::with_capacity(10) ;
	println!("Capacity: {}", ten_positions.capacity()) ; // Output: Capacity: 10
	
	let vector: Vec<i32> = Vec::from(slice) ; // Output: [1, 2, 3, 4, 5]
	
	vector.push(1); 
	vector.push(99);
	println!("{:?}", v); // Output: [1, 2, 3, 4, 5, 1, 99]
	
	let last = vector.pop()
	println!("Last position: {}", last); // Output: [99]
	
	let second = vector.remove(1);
	println!("{:?}", second); // Output: 2 
	println!("{:?}", vector); // Output: [1, 3, 4, 5, 1]
	```
	
	
### Enumerations 
	
- **Definizione**: E' un tipo complesso che può assumere uno tra diversi valori definiti, dove ogni _**variante**_ può contenere dati di un tipo specifico come `String`, `char`, `int`, `float`, ecc.
- **Tipo**: `enum`
- **Caso d'Uso**: Quando si necessita che un'entità possa avere più versioni di se stessa.
- **Esempio**:
	
```Rust
enum EnumName {
	Version1(u8),
	Version2(i8),
	Version3{
		key1:i8, 
		key2:&str, 
		key3:OtherEnum, 
	},
	Version4,
}

let enum_variable1 = EnumName::Version1(9) ;
let enum_variable2 = EnumName::Version2(-9) ;
let enum_variable3 = EnumName::Version3{
key1: 96,
key2: "Questa è roba puzzle!",
key3: OtherEnum::SomeVersion.. ,
} ;
```
	
- **Approfondimento**
	
```Rust
enum Exceptions {
	NoLactose(bool),
	GlutenFree(bool),
	Normal(bool),
	SizeBig(bool),
}

enum Menu { 
	Pizza(String), // Tipo di pizza 
	Pasta { name: String, recipe: Exceptions }, // Tipo di pasta 
	Beverage { name: String, volume: f32 }, // Nome e volume in litri 
	Dessert, // Dolce senza dati associati 
}

/* 
Si seleziona la variante dell'enumerazione con `::`
Ne si dichiara il contenuto
*/

let first_pick = Menu::Pizza(String::from("Prataiola")) ;
let second_pick = Menu::Pasta { 
	name: String::from("Ragù"), 
	recipe: Exceptions::NoLactose(true),
} ;
let second_pick_beverage = Menu::beverage{ 
	name: String::from("Beer"), 
	volume: 0.5
} ;
```
	
>	Da notare la possibilità di creare tipi piramidali complessi con varianti di tipo `enum`
	
#Structs #Enums #Vectors #Strings 
	
---
## **§ Tipi Aggiuntivi**
	
### Slices
	
- **Definizione**: Visualizzazione dinamica di un *array*.
- **Caso d'Uso**: Accedere ad una porzione di un array .
- **Esempio**:
	
```Rust
let array = [1, 2, 3, 4, 5]; 
let slice: &[i32] = &array[1..3]; 
println!("Slice: {:?}", slice);
```

- **Output**: `Slice: [2, 3]`
	
### String Slice  
	
- **Definizione**: (```&str```) Riferimento immutabile ad una stringa
- **Caso d'Uso**: Gestire i dati di testo in maniera temporanea.
- **Esempio**:
	
	```Rust
	let greeting: &str = "Hello"; 
	let mut message = String::from("Hello"); 
	message.push_str(", world!"); 
	println!("{}", message);
	```

- **Output**: `Hello, world!`
	
#Slices #Strings 
	
---
## § Annotazioni Esplicite del Tipo 
	
### Specifiche
	
**Caso d'Uso**: Utilizzo della sintassi `let variabile: tipo = valore`  per limitare e rendere più solido l'utilizzo della memoria ottimizzandola.
	
```Rust
// Implicit Annotiation
let positive_number = 10; // Generates space in memory even for Negative numbers!

// Explicit Anntotation
let positive_number: u8 = 10; // Exclusively for Positive numbers!
```
	
### Generiche
	
**Caso d'Uso**: Si può utilizzare la annotazione anche per i tipi composti e complessi `enum`, `struct`, `vec`.
	
```Rust
enum DirectionalArrows {
	Up,
	Down,
	Left,
	Right,
}

struct Block {
	hash_sign: String,
	merkle_root: String,
	nonce: String,
}


fn main () {
	// ENUMERATIONS
	let run: DirectionalArrow = DirectionalArrows::Up;
	
	// STRUCTURES
	// Blockchain example just to be fancy
	let first_block: Block = {
	    hash_sign: String::from("tBJAihCD4Zc6++SXhcIEn3AqDzfm0x1XrqAa+DmzQh0="),
		merkle_root: String::from("CnPC478PrYguuMwDiaU6BpvC7bTWUMcll+BYhSN2e5k="),
		nonce_count: String::from("00000000000000000"),
	};
	let second_block: Block = {
		hash_sign: String::from("CnPC478PrYguuMwDiaU6BpvC7bTWUMcll+BYhSN2e5k="),
		merkle_root: String::from("sJVxRwy67sCy44Fzen5CT3wmi565pLkuKuqXqcwXkhc="),
		nonce_count: String::from("00000000000000000"),
	};
	
	// VECTORS
	let blockchain: vec<Block> = vec![first_block, second_block]; // STRUCTURES
	let dance: vec<DirectionalArrow> = { // ENUMERATIONS
		DirectionalArrows::Up,
		DirectionalArrows::Left,
		DirectionalArrows::Right,
		DirectionalArrows::Down,
	}

}
```
	
#Enums #Arrays #Vectors  
	
---
##### Progressione Suggerita
[[rust_cheat_sheet_dynamics]]
	
---
	
**Author:** Kenneth Boldrini

