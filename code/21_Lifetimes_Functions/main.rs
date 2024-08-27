/*
Topic: Lifetimes & Functions

Summary:
Write a program that determines which string has the greater length.

Requirements:
* The comparison must be done using a function named `find_longer`.
* No data can be copied (cannot use .to_owned() or .to_string()).
* If both strings have the same length, the first one should be returned.

*/


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}













// MY VERSION------------------------------------------------------------------------------------------------------------------
fn find_longer<'a>(sentence1: &'a str, sentence2: &'a str) -> &'a str { 
    if sentence1.len() >= sentence2.len() {
        sentence1 
    } else {
        sentence2
    }   
}
fn my_main () {
    let short = "hi";
    let long = "this is a much longer message";
    println!("{}", find_longer(short, long));
}

//AI------------------------------------------------------------------------------------------------------------------------
fn find_longer_ai<'a>(s1_ai: &'a str, s2_ai: &'a str) -> &'a str {
    if s1_ai.len() >= s2_ai.len() {
        s1_ai
    } else {
        s2_ai
    }
}
fn ai_main () {
    let short_ai = "hi";
    let long_ai = "this is a much longer message AI says";
    println!("{}", find_longer_ai(short_ai, long_ai));
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