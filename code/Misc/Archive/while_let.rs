fn main()  {
    
    let mut place: Option<&str> = Some("Home") ;

    while let Some("Home") = place {
        println!("Relax");
        place = None; // Exit loop
    }

    let animals: [Result<&str, &str>; 5] = [
        Ok("Cat"),
        Ok("Lion"),
        Ok("Dog"),
        Ok("Shark"),
        Err("No animal in queue")
    ] ;

    let mut veterinary_inspection = animals.iter() ;

    // Cicla finche è disponibile un valore Some(_)
    while let Some(Ok(_)) = veterinary_inspection.next() {
        println!("Visited");
    }

}