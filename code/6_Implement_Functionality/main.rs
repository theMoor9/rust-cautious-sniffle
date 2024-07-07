/*
Topic: Implementing functionality with the impl keyword

Requirements:
* Print the attributes of a delivery car
* Must include dimensions, mass, and color

Notes:
* Use a struct to encapsulate the car attributes
* Use an enum for the car color
* Implement functionality on the struct to create a new car
* Implement functionality on the struct to print the attributes
*/



//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {
}

// MY VERSION------------------------------------------------------------------------------------------------------------------
struct Car {
    brand: String,
    length: u8,
    height: u8,
    width: u8,
    weight: u8,
    color: Color,
}

enum Color {
    Green, 
    Red, 
    Blue,
}

impl Car {
    fn create_car (brand: String, length: u8, height: u8, width: u8, weight: u8, color: Color) -> Self {
        Car {
            brand,
            length,
            height,
            width,
            weight,
            color,
        }
    }
    fn print_car (&self) {
        println!("Brand: {}",self.brand);
        println!("Length: {}",self.length);
        println!("Height: {}",self.height);
        println!("width: {}",self.width);
        println!("Weight: {}",self.weight);
        match &self.color {
            Color::Green => println!("Color: Green"),
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
        }
    }
}

fn my_main () {
    let alfa_romeo = Car::create_car(String::from("Alfa Romeo"),10,10,10,10, Color::Blue);
    Car::print_car(&alfa_romeo);
}


//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    your_main();
    my_main();
}
