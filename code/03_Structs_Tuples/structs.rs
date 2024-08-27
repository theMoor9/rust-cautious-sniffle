/*
Topic: Oganinizingsimilar data usining structs

Requirements:
* Print the tipe of cocktail and it's volume
  
Notes:
* Use an enum to create different flavors 
* Use a struct to store flavors and volume information
* Use a function to print out the fields information
* Use a match expression to print the drink flavor
*/

//YOUR VERSION----------------------------------------------------------------------------------------------------------------
fn your_main () {

}

// MY VERSION------------------------------------------------------------------------------------------------------------------
enum Flavor {
    Gintonic,
    Sidecar,
    Martini,
}

struct Drink {
    flavor: Flavor,
    volume: f32,
}

fn kind_of_drink (drink: Drink) {
    match drink.flavor{
        Flavor::Gintonic => println!("\nI choose a Gintonic Vol:{}\n",drink.volume),
        Flavor::Sidecar => println!("\nI choose a Sidecar Vol:{}\n",drink.volume),
        Flavor::Martini => println!("\nI choose a Martini Vol:{}\n",drink.volume),
    }
}

fn my_main () {

    let choice = Drink {
        flavor: Flavor::Sidecar,
        volume: 40.0,
    };

    kind_of_drink(choice)
    
}

//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    your_main();
    my_main();

}
