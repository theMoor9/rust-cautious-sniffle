/* 
Topic: Result

Requirements:
* Create a structure named 'Person' that represents a person aged 21 or older:
    * The structure must contain the person's name and age
    * Implement Debug print functionality using `derive`
* Implement a `new` function for the 'Person' structure that returns a Result:
    * The Ok variant should contain the initialized structure, but only
        if the person is aged 21 or older
    * The Err variant should contain a String (or &str) that explains why
        the structure could not be created
* Instantiate two 'Person' structures:
    * One should be aged under 21
    * One should be 21 or over
* Use `match` to print out a message for each 'Person':
    * For the Ok variant, print any message you want
    * For the Err variant, print out the error message
*/

//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
#[derive(Debug)]
struct Person {
    name: String,
    age:u8,
}

impl Person {
    fn new ( anagraphics: (String, u8)) -> Result<Person,(String,String,u8)> {
        if anagraphics.1 > 21 {
            Ok(Person{name: anagraphics.0, age:anagraphics.1})
        } else {
            Err(("Too young".to_owned(), anagraphics.0, anagraphics.1))
        }   
    }
} 


fn my_main () {

    let me = ("Kenneth".to_owned(), 27);
    let her = ("Francesca".to_owned(), 20);
    let count: u64 = 00;

    let person1: (Result<Person,(String,String,u8)>, u64) = (Person::new(me), count + 1);
    match person1 {
        (Ok(who),c) => println!("{} ---\n{} is old enough with age: {} \nPerson added succesfully!", c, who.name, who.age),
        (Err(e),c) => println!("{} ---\n{} is {} with age: {}", c, e.1, e.0, e.2),
    } 
    
    let person2: (Result<Person,(String,String,u8)>, u64) = (Person::new(her), count + 2);
    match person2 {
        (Ok(who),c) => println!("{} ---\n{} is old enough with age: {}", c, who.name, who.age),
        (Err(e),c) => println!("{} ---\n{} is {} with age: {}", c, e.1, e.0, e.2),
    }

}

//AI------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct PersonAI {
    name: String,
    age: u8,
}

impl PersonAI {
    fn new(name: String, age: u8) -> Result<PersonAI, String> {
        if age >= 21 {
            Ok(PersonAI { name, age })
        } else {
            Err(format!("{} is too young to be considered an adult", name))
        }
    }
}

fn ai_main() {
    let person1_ai = PersonAI::new(String::from("Alice"), 20);
    let person2_ai = PersonAI::new(String::from("Bob"), 22);

    match person1_ai {
        Ok(person) => println!("Created person: {:?}", person),
        Err(e) => println!("Error: {}", e),
    }

    match person2_ai {
        Ok(person) => println!("Created person: {:?}", person),
        Err(e) => println!("Error: {}", e),
    }
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