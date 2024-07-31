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
	let input = 1;
	let correction = 6;
	
	let sum_corrected = sum(1,1,Some(input)).map(|sum_result| sum_result + correction);

    match sum_corrected {
        Ok(sum_r) => println!("Result: {}", sum_r),
        Err(e) => println!("Error: {}", e)
    }
	
}