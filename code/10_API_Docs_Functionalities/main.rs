 /* 
 Topic: Browsing the standard library documentation

 Requirements:
 * Print a string in uppercase and lowercase

 Notes:
 * Use standard library functionality to transform the string to uppercase and lowercase
 * Use 'rustup doc' in a terminal to open the standard library documentation
 * Navigate to the API documentation section
 * Search for functionality to transform a string (or str) to uppercase and lowercase
 * Try searching for: to_uppercase, to_lowercase
*/


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {
}

// MY VERSION------------------------------------------------------------------------------------------------------------------
fn uppercase_print (count: &u32, string: &String) {
    println!("{} ---\nString: {}", count, string);
    println!("Uppercase: {}", string.to_uppercase());
    println!("Lowercase: {}", string.to_lowercase());
}

fn my_main () {

    let mut name: String = String::from("KeNnEth");
    let mut count: u32 = 01;

    uppercase_print( &count, &name );
    count = count+1;
    name =  String::from("PhIlIp");
    uppercase_print( &count, &name );

}

//AI------------------------------------------------------------------------------------------------------------------------

fn ai_main () {
    let example = "Hello, World!";
    
    // Convert the string to uppercase
    let uppercased = example.to_uppercase();
    
    // Convert the string to lowercase
    let lowercased = example.to_lowercase();
    
    println!("Uppercase: {}", uppercased);
    println!("Lowercase: {}", lowercased);
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