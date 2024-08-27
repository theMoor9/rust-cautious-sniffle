// Topic: Expressions
//
// Requirements:
// * print "It's BIG" if a variable is < 50
// * print "It's small" if a variable is >= 50
//
// Notes:
// * Use a boolean variable set to an expression that
//   determines whether the value is < 50 or >= 50
// * Use a function to print the messages
// * Use a match expression to determine which message to print


//YOUR VERSION-----------------------------------------------------------------------------------------------------------------
fn your_main () {
   
}

// MY VERSION------------------------------------------------------------------------------------------------------------------
fn my_main (value: i32) {
   
    let mut check: bool = false;
    let mut control: bool = false; 

    fn determine_size(size_check: bool, control: bool){
        if control {
            match size_check {
                true => println!("It's BIG"),
                false => println!("It's small"),
            }
        } else {
            println!("It's not a valid size") ;
        }
    }

    if value < 50 {
        check = false ;
        control = true ;
    } else if value >= 50 {
        check = true ;
        control = true ;
    } else {
        control = false ;
    }

    determine_size(check, control) ;

}

//MAIN------------------------------------------------------------------------------------------------------------------------
fn main () {
    let mut v = 50 ;
    //your_main(v) ;
    my_main(v) ;
}

/* 
Personal Comment:
The `control` variable is superfluous in Rust thanks to the robustness of the language,
which guarantees the validity of the `value` as an `i32`. However, 
I kept it to show it in a more meaningful way. 
For a more idiomatic approach, you could directly use an
expression to set the boolean value and further simplify the code. 

let mut check = value >= 50 ; // Getting the expression directly in the variable
*/


