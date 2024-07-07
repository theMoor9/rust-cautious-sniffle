/*
Topic: Tuples

Requirements:
* Print whether the y value of a cartesian coordinate is >9,
  <9 or =9.

Notes:
* Use a function that returns a tuple.
* Unpack the tuple in two variables.
* Use an if-else to determine what to print.

*/


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {

}

// MY VERSION------------------------------------------------------------------------------------------------------------------
// Use a function that returns a tuple.
fn generate_coordinates () -> (i64, i64) {
    // User input
    let input_x: i64 = 10 ;
    let input_y: i64 = 9 ;
    println!("Input: X:{}, Y:{}", input_x, input_y);

    return(input_x, input_y)
}

fn my_main () {
    
    let mut control: bool = true ;
    // Unpack the tuple in two variables.
    let (_x,y) = generate_coordinates() ;

    //Use an if-else statement to determine what to print.
    while control {
        if y > 9 {
            println!("Y is grater than 9\n") ;
            control = false ;
        } else if y < 9 {
            println!("Y is less than 9\n") ;
            control = false ;
        } else {
            println!("Y is equal to 9\n") ;
            control = false ;
        }
    }
}


//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main( ) {
    your_main();
    my_main();
}

