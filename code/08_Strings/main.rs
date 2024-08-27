/*
Topic: Strings

 Requirements:
 * Print out the name and preferred colors of individuals aged 12 and below

 Notes:
 * Use a struct for a person's age, name, and preferred color
 * The color and name should be stored as a String
 * Create and store at least 3 individuals in a vector
 * Iterate through the vector using a for..in loop
 * Use an if expression to determine which individual's info should be printed
 * The name and colors should be printed using a function
*/




//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {
}

// MY VERSION------------------------------------------------------------------------------------------------------------------

struct Human {
    name: String,
    age: u8,
    color: String,
}

impl Human {

    fn new (name: String, age: u8, color: String) -> Self {
        Human{
            name,
            age,
            color,
        }
    }

    fn tell_color (&self) {
        println!("{}'s favourite color is {}!",self.name ,self.color );
    }
}

fn my_main () {
    let mut individuals: Vec<Human> = Vec::with_capacity(3);

    let person1 = Human::new(String::from("Kenneth"), 27, String::from("Purple"));
    let person2 = Human::new(String::from("Philip"), 22, String::from("Green"));
    let person3 = Human::new(String::from("Linda"), 25, String::from("Black"));

    individuals.push(person1);
    individuals.push(person2);
    individuals.push(person3);

    for soul in individuals {
        if soul.name == "Linda"{
            Human::tell_color(&soul);
        }
    }
}

//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    your_main();
    my_main();
}

