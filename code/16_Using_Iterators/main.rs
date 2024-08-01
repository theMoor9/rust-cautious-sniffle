/*
 Topic: Using Iterators
 
 Requirements:
 * Filtra e somma solo i numeri dispari da un vettore di interi.
 
 Notes:
 * Utilizza gli iteratori per filtrare e sommare gli elementi.
 * Usa `filter` per selezionare solo numeri dispari e `sum` per sommare.
  
 Example:
 * Input: vec![1, 2, 3, 4, 5]
 * Output: 9 (somma di 1, 3, e 5)
 */


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------

fn my_main () {
    // You can find my practice exercises in ../Misc/Archive
}

//AI------------------------------------------------------------------------------------------------------------------------

fn ai_main() {
    let ai_numbers = vec![1, 2, 3, 4, 5];
    let ai_odd_sum: i32 = ai_numbers.iter().filter(|&&x| x % 2 != 0).sum();
    println!("Sum of odd numbers: {}", ai_odd_sum);
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