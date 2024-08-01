/*
 Topic: `map` Combinator
 
 Requirements:
 * Converti un vettore di numeri interi in stringhe che rappresentano ciascun numero.
 * Stampa il vettore risultante.
 
 Notes:
 * Utilizza il combinator `map` per trasformare ogni elemento del vettore.
 * Utilizza `collect()` per raccogliere i risultati in un nuovo vettore.
 
 Example:
 * Input: vec![1, 2, 3, 4]
 * Output: vec!["1", "2", "3", "4"]
 */


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------

fn my_main () {
    // You can find my practice exercises in ../Misc/Archive
}

//AI------------------------------------------------------------------------------------------------------------------------

fn ai_main() {
    let ai_numbers = vec![1, 2, 3, 4];
    let ai_string_numbers: Vec<String> = ai_numbers.iter().map(|&num| num.to_string()).collect();
    println!("{:?}", ai_string_numbers);
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