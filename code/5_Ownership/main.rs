/*
Topic: Owneship

Requirements:
* Print out the quantity and Price of a shop item

Notes:
* Use a struct for the shop item
* Use two i32 fields for the quantity and Price
* Create a function to display the quantity
* Create a function to display the Price
*/


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {
}

// MY VERSION------------------------------------------------------------------------------------------------------------------
// Use a struct for the shop item
struct ShopItem {
    name: String,
    // Use two i32 fields for the quantity and Price
    quantity: i32,
    price: i32,
}

// Create a function to display the quantity
fn display_amount (item: &ShopItem) {
    println!("You got {} {}\n", item.quantity, item.name) ;
}
// Create a function to display the Price
fn display_id (item: &ShopItem) {
    println!("\n{} is worth: {}€", item.name, item.price) ;
}

fn my_main () {

    let chosen_item = ShopItem {
        name: String::from("Apple"),
        quantity: 9,
        price: 906,
    };
    
    display_id(&chosen_item) ;
    display_amount(&chosen_item) ;
    
}

//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    your_main();
    my_main();
}

