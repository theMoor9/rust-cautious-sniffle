fn main() {
    let place = Some("Vallombrosa");
    match place {
        Some(p) => println!("You are in: {}", p), // Output: You are in: Vallombrosa
        None => println!("I really dont't care where you are")
    }

    if let Some(p) = place {
        println!("You are in: {}", p); // Output: You are in: Vallombrosa
    }

    // E' possibile inoltre replicare la struttuara match aggiungendo un else

    if let Some(p) = place {
        println!("You are in: {}", p); // Output: You are in: Vallombrosa
    } else {
        println!("I really dont't care where you are");
    }
}