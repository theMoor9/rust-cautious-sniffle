enum MenuChoice{
    FirstDish(String),
    SecondDish(String),
    Dessert(String),
    Drink(String),
}

fn main () {

    let choice = MenuChoice::SecondDish(String::from("Fiorentina steak"));

    match choice {
        MenuChoice::FirstDish(ref pick) => println!("He ordered {}", pick),
        MenuChoice::SecondDish(ref pick) => println!("He ordered {}", pick),
        MenuChoice::Dessert(ref pick) => println!("He ordered {}", pick),
        MenuChoice::Drink(ref pick) => println!("He ordered {}", pick),
    };
    
    // Othewise
    
    fn matching_function (preference: MenuChoice) {
        match preference{
            MenuChoice::FirstDish(ref pick) => println!("He ordered {}", pick),
            MenuChoice::SecondDish(ref pick) => println!("He ordered {}", pick),
            MenuChoice::Dessert(ref pick) => println!("He ordered {}", pick),
            MenuChoice::Drink(ref pick) => println!("He ordered {}", pick),
        }
    }
    
    matching_function(choice); // Fucntion call on choice

}

/* 
Personal Comment:

This is two of two exercises that is not to be completed, sry.
Other exercises will be settled for you to be used

*/