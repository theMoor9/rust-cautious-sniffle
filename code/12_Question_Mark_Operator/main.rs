/*
 Topic: Result and the Question Mark Operator
 
 Summary:
 This program simulates unlocking a door using digital keycards
 backed by a database. Many errors can occur when working with a database,
 making the question mark operator the perfect tool to keep the code manageable.
 
 Requirements:
 * Write the body of the `authorize` function. The steps to authorize a user are:
 1. Connect to the database
 2. Find the employee using the database function `find_employee`
 3. Get a keycard using the database function `get_keycard`
 4. Determine if the keycard's access level is sufficient using the
   `required_access_level` function implemented on `ProtectedLocation`.
    Higher `access_level` values grant more access to `ProtectedLocations`.
    1000 can access 1000 and lower, 800 can access 500 but not 1000, etc.
 * Run the program after writing the `authorize` function. Expected output:
    Ok(Allow)
    Ok(Deny)
    Err("Catherine doesn't have a keycard")
    Use the question mark operator within the `authorize` function.
 
 Notes:
 Only the `authorize` function should be changed. Everything else can remain
 unmodified.
 */


//YOUR VERSION----------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum ProtectedLocation1 {
    Office,
    ServerRoom,
}

impl ProtectedLocation1 {
    fn required_access_level(&self) -> u32 {
        match self {
            ProtectedLocation1::Office => 500,
            ProtectedLocation1::ServerRoom => 1000,
        }
    }
}

struct Employee1 {
    id: u32,
    name: String,
    access_level: u32,
}

struct Keycard1 {
    employee_id: u32,
    access_level: u32,
}

fn connect_to_database1() -> Result<(), String> {
    // Simulated database connection
    Ok(())
}

fn find_employee1(id: u32) -> Result<Employee1, String> {
    // Simulated database lookup
    if id == 1 {
        Ok(Employee1 { id, name: "Alice".to_string(), access_level: 800 })
    } else if id == 2 {
        Ok(Employee1 { id, name: "Bob".to_string(), access_level: 500 })
    } else {
        Err("Employee not found".to_string())
    }
}

fn get_keycard1(employee_id: u32) -> Result<Keycard1, String> {
    // Simulated database lookup
    if employee_id == 1 {
        Ok(Keycard1 { employee_id, access_level: 800 })
    } else if employee_id == 2 {
        Ok(Keycard1 { employee_id, access_level: 500 })
    } else {
        Err("Keycard not found".to_string())
    }
}

// This function needs to be implemented
fn authorize1(employee_id: u32, location: ProtectedLocation1) -> Result<String, String> {
    // Your code goes here
    unimplemented!()
}

fn your_main() {
    let result = authorize1(1, ProtectedLocation1::ServerRoom);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize1(2, ProtectedLocation1::Office);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize1(3, ProtectedLocation1::Office);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
}

// MY VERSION------------------------------------------------------------------------------------------------------------------


#[derive(Clone, Copy, Debug)]
enum ProtectedLocation2 {
    Office,
    ServerRoom,
}

impl ProtectedLocation2 {
    fn required_access_level2(&self) -> u32 {
        match self {
            ProtectedLocation2::Office => 500,
            ProtectedLocation2::ServerRoom => 1000,
        }
    }
}

struct Employee2 {
    id: u32,
    name: String,
    access_level: u32,
}

struct Keycard2 {
    employee_id: u32,
    access_level: u32,
}

fn connect_to_database2() -> Result<(), String> {
    // Simulated database connection
    Ok(())
}

fn find_employee2(id: u32) -> Result<Employee2, String> {
    // Simulated database lookup
    if id == 1 {
        Ok(Employee2 { id, name: "Kenneth".to_string(), access_level: 800 })
    } else if id == 2 {
        Ok(Employee2 { id, name: "Francesca".to_string(), access_level: 500 })
    } else {
        Err("Employee not found".to_string())
    }
}

fn get_keycard2(employee_id: u32) -> Result<Keycard2, String> {
    // Simulated database lookup
    if employee_id == 1 {
        Ok(Keycard2 { employee_id, access_level: 800 })
    } else if employee_id == 2 {
        Ok(Keycard2 { employee_id, access_level: 500 })
    } else {
        Err("Keycard not found".to_string())
    }
}

// This function needs to be implemented
fn authorize2(employee_id: u32, location: ProtectedLocation2) -> Result<String, String> {
    // Your code goes here
    let db = connect_to_database2()?;
    let person = find_employee2(employee_id)?;
    let persons_keycard = get_keycard2(employee_id)?;

    let level: u32 = location.required_access_level2(); //Rerturns the required level

    //comparison between keycard.access_level and level.by match
    if persons_keycard.access_level >= level {
        Ok("Allow".to_string())
    } else { 
        Ok(String::from("Deny")) 
    }
}

fn my_main() {
    let result = authorize2(1, ProtectedLocation2::ServerRoom);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize2(2, ProtectedLocation2::Office);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize2(3, ProtectedLocation2::Office);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
}


//AI------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
    Office,
    ServerRoom,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u32 {
        match self {
            ProtectedLocation::Office => 500,
            ProtectedLocation::ServerRoom => 1000,
        }
    }
}

struct Employee {
    id: u32,
    name: String,
    access_level: u32,
}

struct Keycard {
    employee_id: u32,
    access_level: u32,
}

fn connect_to_database() -> Result<(), String> {
    // Simulated database connection
    Ok(())
}

fn find_employee(id: u32) -> Result<Employee, String> {
    // Simulated database lookup
    if id == 1 {
        Ok(Employee { id, name: "Alice".to_string(), access_level: 800 })
    } else if id == 2 {
        Ok(Employee { id, name: "Bob".to_string(), access_level: 500 })
    } else {
        Err("Employee not found".to_string())
    }
}

fn get_keycard(employee_id: u32) -> Result<Keycard, String> {
    // Simulated database lookup
    if employee_id == 1 {
        Ok(Keycard { employee_id, access_level: 800 })
    } else if employee_id == 2 {
        Ok(Keycard { employee_id, access_level: 500 })
    } else {
        Err("Keycard not found".to_string())
    }
}

fn authorize(employee_id: u32, location: ProtectedLocation) -> Result<String, String> {
    connect_to_database()?;
    let employee = find_employee(employee_id)?;
    let keycard = get_keycard(employee_id)?;

    if keycard.access_level >= location.required_access_level() {
        Ok("Allow".to_string())
    } else {
        Ok("Deny".to_string())
    }
}

fn ai_main() {
    let result = authorize(1, ProtectedLocation::ServerRoom);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize(2, ProtectedLocation::Office);
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
    
    let result = authorize(3, ProtectedLocation::Office);
    match result {
        Ok(message) => println!("{}", message),
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

This has been hard for me, the compiler got me pants down multple times. 
I took me at least 1hr to complete. T_T

*/