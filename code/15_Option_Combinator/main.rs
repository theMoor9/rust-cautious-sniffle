/*
 Topic: `Option` Combinators
  
 Requirements:
 * Data una stringa che rappresenta un numero intero, convertila in `i32` e moltiplica il risultato per 2.
 * Gestisci il caso in cui la conversione fallisce, restituendo 0 come valore di default.
 
 Notes:
 * Utilizza `map` per trasformare il valore contenuto nell'`Option`.
 * Utilizza `unwrap_or` per gestire il caso di errore.
 
 Example:
 * Input: "42"
 * Output: 84
 * Input: "abc"
 * Output: 0
 */


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------

fn my_main () {
    // You can find my practice exercises in ../Misc/Archive
}

//AI------------------------------------------------------------------------------------------------------------------------
fn ai_main() {
    let ai_input = "42";
    let ai_result = ai_input.parse::<i32>().map(|n| n * 2).unwrap_or(0);
    println!("Result: {}", ai_result);
}

//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    println!("\nYou:");
    your_main();
    println!("\nMe:");
    my_main();
    println!("\nAI:");
    ai_main();
    println!("");
}

/* 
Personal Comment post verification:

*/