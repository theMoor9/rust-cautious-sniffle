/*
Topic: Module Implementation in Rust

Requirements:
* Create a module named `math_operations`.
* Inside the module, define three functions:
  1. Create a function that returns the sum of two numbers.
  2. Create a function that returns the product of two numbers.
  3. Create a function that subtracts the second number from the first and is private.
* In the `main` function, demonstrate the use of `add` and `multiply`.
* Try to use the `subtract` function from `main` to illustrate its inaccessibility due to being private.

Notes:
* This exercise helps understand the encapsulation of functionality within modules, the use of public and private access modifiers, and how to structure a Rust program using modules for better organization and maintainability.
* Discuss why encapsulating functions in a module can be beneficial in larger software projects.
* Make sure to handle any errors or edge cases if applicable.
*/


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
mod math_ops {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn mult(a: i32, b: i32) -> i32 {
        a * b
    }
    pub fn mult_sub(a: i32, b: i32, c: i32) -> i32 {
        let r = mult(a,b);
        sub(r,c)
    }

    // Sub can be used inside math_ops
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

}

fn my_main () {
    use math_ops::*;
    println!("{}",add(3,6)); // Output: 9
    println!("{}",mult(1,3)); // Output: 3
    
    // println!("{}",sub(1,3)); // Compiler's angry
    println!("{}",mult_sub(1,3,12)); // Output: -9
}

//AI------------------------------------------------------------------------------------------------------------------------

mod ai_math_operations {
    pub fn ai_add(ai_a: i32, ai_b: i32) -> i32 {
        ai_a + ai_b
    }

    pub fn ai_multiply(ai_a: i32, ai_b: i32) -> i32 {
        ai_a * ai_b
    }

    fn ai_subtract(ai_a: i32, ai_b: i32) -> i32 {
        ai_a - ai_b
    }
}

fn ai_main() {
    let ai_result_add = ai_math_operations::ai_add(10, 5);
    println!("Addition result: {}", ai_result_add);

    let ai_result_multiply = ai_math_operations::ai_multiply(10, 5);
    println!("Multiplication result: {}", ai_result_multiply);

    // Uncommenting the following line will result in a compilation error because ai_subtract is private
    // let ai_result_subtract = ai_math_operations::ai_subtract(10, 5);
    // println!("Subtraction result: {}", ai_result_subtract);
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