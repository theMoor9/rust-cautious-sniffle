/*
Topic: Traits

Requirements:
- Calculate and print the perimeter of a square and a triangle.
- The perimeter of a square is calculated by multiplying the length of one side by 4.
- The perimeter of a triangle is obtained by adding the lengths of its three sides.

Notes:
- Use a trait to declare a function for perimeter calculation.
- Use a single function to print the perimeter of the shapes, using the trait as a function parameter.
*/



//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
enum Shapes{
    Square,
    Triangle
}
struct Square {
    side_len: u64,
    id: Shapes
}
struct Triangle {
    side1_len: u64,
    side2_len: u64,
    side3_len: u64,
    id: Shapes
}

trait Perimeter {
    fn calc(&self) -> u64;
}

impl Perimeter for Square {
    fn calc(&self) -> u64 {
        println!("Square's side is {}",self.side_len);
        self.side_len * 4
    }
}
impl Perimeter for Triangle {
    fn calc(&self) -> u64 {
        println!("Triangle's side are {},{},{}",
            self.side1_len,
            self.side2_len,
            self.side3_len
        );
        self.side1_len + self.side2_len + self.side3_len
    }
}

fn print_perimeter(shape: & impl Perimeter, id: & Shapes) {
    match id {
        Shapes::Square => println!("The Square perimeter is: {}\n",shape.calc()),
        Shapes::Triangle => println!("The Triangle perimeter is: {}\n",shape.calc()),
    }
    
}

fn my_main() {
    let triangle = Triangle{
        side1_len:3,
        side2_len:3,
        side3_len:3,
        id: Shapes::Triangle
    };
    let square = Square {side_len: 1, id: Shapes::Square};

    print_perimeter(&square, &square.id);
    print_perimeter(&triangle, &triangle.id);

}

//AI------------------------------------------------------------------------------------------------------------------------
// Step 1: Definire un Trait per il calcolo del perimetro
trait PerimeterAi {
    fn calc_perimeter_ai(&self) -> f64;
}
// Step 2: Creare le strutture Square e Triangle
struct SquareAi {
    side_len_ai: f64,
}

impl PerimeterAi for SquareAi {
    fn calc_perimeter_ai(&self) -> f64 {
        self.side_len_ai * 4.0
    }
}

struct TriangleAi {
    side1_len_ai: f64,
    side2_len_ai: f64,
    side3_len_ai: f64,
}

impl PerimeterAi for TriangleAi {
    fn calc_perimeter_ai(&self) -> f64 {
        self.side1_len_ai + self.side2_len_ai + self.side3_len_ai
    }
}

// Step 3: Creare una funzione che accetta il trait come parametro
fn print_perimeter_ai(shape_ai: impl PerimeterAi) {
    let perimeter_ai = shape_ai.calc_perimeter_ai();
    println!("The perimeter is: {}", perimeter_ai);
}
fn ai_main () {
    let square_ai = SquareAi { side_len_ai: 5.0 };
    let triangle_ai = TriangleAi {
        side1_len_ai: 3.0,
        side2_len_ai: 4.0,
        side3_len_ai: 5.0,
    };

    print_perimeter_ai(square_ai);
    print_perimeter_ai(triangle_ai);
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