/*
 Topic: Vectors

 Requirements:
 * Add 4 numbers (7, 14, 21, 28) to a vector
 * Print 7, 14, "twenty-one", and 28 in a loop
 * Print the total number of items in the vector

 Notes:
 * Use a vector to store 4 items
 * Iterate through the vector using a for..in loop
 * Decide whether to print the number or print "twenty-one" inside the loop
 * Use the .len() function to print the number of items
*/



//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {

}

// MY VERSION------------------------------------------------------------------------------------------------------------------

fn my_main () {

    let mut sundays: Vec<u8> = Vec::new();
    let mut day: u8 = 7;

    while day <= 28 {
        sundays.push(day);
        day = day + 7;
    }

    for days in &sundays {
        match days {
            21 => println!("Sunday Twenty-one"),
            _ => println!("Sunday {}", days), 
        }
    }
    
    println!("Sundays in this month: {}", sundays.len());
}



//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    your_main();
    my_main();
}

