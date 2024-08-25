/*
Topic: Generic Structures

Requirements:
- Create a structure for a generic vehicle that can have different bodies and colors.
- Create structures for different vehicle bodies and colors, implementing traits for each structure.
- Implement a `new` function for the vehicle structure that allows assigning any body and color.
- Create at least two different vehicles and print their information.

Notes:
- Examples of vehicle bodies: Truck, Car, Scooter.
- Examples of colors: Red, White, Black.
*/



//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
enum Bodies {
    Truck,
    Car,
    Scooter,
}
enum Colors {
    Red,
    White,
    Black,
    Blue
}
trait BodiesShow {
    fn show(&self) -> &str;
}
trait ColorsShow {
    fn show(&self) -> &str;
}

impl BodiesShow for Bodies {
    fn show(&self) -> &str {
        match self {
            Bodies::Truck => "Truck",
            Bodies::Car => "Car",
            Bodies::Scooter => "Scooter",
        }
    }
}
impl ColorsShow for Colors {
    fn show(&self) -> &str {
        match self {
            Colors::Red => "Red",
            Colors::White => "White",
            Colors::Black => "Black",
            Colors::Blue=> "Blue",
        }
    }
}

struct Veichle<T: BodiesShow, U: ColorsShow> {
    body: T,
    color: U
}
impl<T: BodiesShow, U: ColorsShow> Veichle<T, U> {
    fn new(b: T, c: U) -> Self {
        Self {
            body: b,
            color: c
        }
    }
}

fn my_main () {
    let my_color = Colors::Blue;
    let her_color = Colors::Black;

    let my_preference = Bodies::Car;
    let her_preference = Bodies::Scooter;

    let my_veichle = Veichle::new(my_preference,my_color);
    let her_veichle = Veichle::new(her_preference,her_color);

    println!("My {} is {}!",my_veichle.body.show(),my_veichle.color.show());
    println!("Her {} is {}!",her_veichle.body.show(),her_veichle.color.show());  
}


//AI------------------------------------------------------------------------------------------------------------------------
// Step 1: Definire i Trait per Carrozzeria e Colore
trait BodyAi {
    fn body_type_ai(&self) -> &str;
}

trait ColorAi {
    fn color_name_ai(&self) -> &str;
}

// Step 2: Implementare le Strutture per Carrozzerie
struct TruckAi;
struct CarAi;
struct ScooterAi;

impl BodyAi for TruckAi {
    fn body_type_ai(&self) -> &str {
        "Truck"
    }
}

impl BodyAi for CarAi {
    fn body_type_ai(&self) -> &str {
        "Car"
    }
}

impl BodyAi for ScooterAi {
    fn body_type_ai(&self) -> &str {
        "Scooter"
    }
}

// Step 2: Implementare le Strutture per Colori
struct RedAi;
struct WhiteAi;
struct BlackAi;

impl ColorAi for RedAi {
    fn color_name_ai(&self) -> &str {
        "Red"
    }
}

impl ColorAi for WhiteAi {
    fn color_name_ai(&self) -> &str {
        "White"
    }
}

impl ColorAi for BlackAi {
    fn color_name_ai(&self) -> &str {
        "Black"
    }
}

// Step 3: Creare la Struttura Generica Vehicle
struct VehicleAi<B: BodyAi, C: ColorAi> {
    body_ai: B,
    color_ai: C,
}

// Step 4: Implementare la Funzione new
impl<B: BodyAi, C: ColorAi> VehicleAi<B, C> {
    fn new_ai(body_ai: B, color_ai: C) -> Self {
        Self { body_ai, color_ai }
    }

    fn print_info_ai(&self) {
        println!("Vehicle: {} in {}", self.body_ai.body_type_ai(), self.color_ai.color_name_ai());
    }
}
fn ai_main () {
    let red_truck_ai = VehicleAi::new_ai(TruckAi, RedAi);
    let white_car_ai = VehicleAi::new_ai(CarAi, WhiteAi);
    let black_scooter_ai = VehicleAi::new_ai(ScooterAi, BlackAi);

    red_truck_ai.print_info_ai();
    white_car_ai.print_info_ai();
    black_scooter_ai.print_info_ai();
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