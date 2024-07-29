/*
Topic: HashMap

Requirements:
* Print the name and number of items in stock for a furniture store
* If the number of items is 0, print "out of stock" instead of 0
* The store has:
    - 5 Chairs
    - 3 Beds
    - 2 Tables
    - 0 Couches
* Print the total number of items in stock

Notes:
* Use a HashMap for the furniture store stock
*/
use std::collections::HashMap;


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
struct Stock {
    product: String,
    qty: u64,
}

fn my_main () {

    println!("\nSTORE:");

    //Use a HashMap for the furniture store stock
    let mut store: HashMap<u64,Stock> = HashMap::new();
    let mut total: u64 = 0;

    //The store has:
    store.insert(0,Stock {product: String::from("Chair"), qty: 5});
    store.insert(1,Stock {product: String::from("Beds"), qty: 3});
    store.insert(2,Stock {product: String::from("Tables"), qty: 2});
    store.insert(3,Stock {product: String::from("Couches"), qty: 0});

    //Print the name and number of items in stock for a furniture store
    for (id, s) in store.iter() {
        //If the number of items is 0, print "out of stock" instead of 0
        if s.qty == 0 {
            println!("ID{} | {} out of stock", id, s.product);
        // Print the name and number of items in stock for a furniture store
        } else {
            println!("ID{} | {} quantity: {}", id, s.product, s.qty);
        }

        total = total + s.qty

    }

    //Print the total number of items in stock
    println!("Total items in stock: {}", total);

}















//AI------------------------------------------------------------------------------------------------------------------------

fn ai_main() {
    // Creiamo una HashMap per memorizzare il magazzino del negozio di mobili
    let mut stock: HashMap<&str, i32> = HashMap::new();
    
    // Inseriamo gli articoli nel magazzino
    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);
    
    // Inizializziamo il contatore per il totale degli articoli in magazzino
    let mut total_items = 0;

    // Stampiamo il nome e il numero di articoli in magazzino
    for (item, &count) in &stock {
        if count == 0 {
            println!("{}: out of stock", item);
        } else {
            println!("{}: {}", item, count);
        }
        total_items += count;
    }
    
    // Stampiamo il totale degli articoli in magazzino
    println!("Total number of items in stock: {}", total_items);
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