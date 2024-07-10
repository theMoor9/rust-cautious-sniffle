# Rust Cheat Sheet - Macro
Funzioni *imbedded* distinte dall'aggiunta di `!` prima dei loro argomenti `(...)`
	
---
## println!
	
Oltre il classico uso, `println!` ha bisogno elementi *`token`* da usare nei placeholders `{...}` come argomenti oltre la stringa per stampare variabili
	
```Rust
let token = value
println!("string {}", token)
// Sostituisce {} con il valore di token_value
```
	
	
**Tipologie di placeholders:**
	
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
## vec!
	
Macro che consente la generazione dei vettori di tipo univoco, a cui si applica tutte le proprietà delle *struct* `Vec` (Vedi [[rust_cheat_sheet_types]]).
	
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

```
	
	
---
	
**Author:** Kenneth Boldrini
