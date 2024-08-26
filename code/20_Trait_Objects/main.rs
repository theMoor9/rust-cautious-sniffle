/*
Topic: Trait Objects

Requirements:
- Create a program that calculates the total cost of materials for a job based on the required square meters.
- Calculate the total cost using various types of materials with different costs per square meter.
- Possible materials:
  - Carpet: $10 per square meter
  - Tile: $15 per square meter
  - Wood: $20 per square meter

Notes:
- Create a trait that retrieves the cost for each type of material.
- Use a function to calculate the total cost, processing a list of different materials.
- Take into account the quantity in square meters for each material.
*/



//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {}

// MY VERSION------------------------------------------------------------------------------------------------------------------
/*
Gli elementi che necessitano di un tratto che li renda dinamici in runtime sono:
- Carpet
- Tile
- Wood
*/
struct Carpet{
  name: String,
  price: u64,
  m2: u64
}
struct Tile{
  name: String,
  price: u64,
  m2: u64
}
struct Wood{
  name: String,
  price: u64,
  m2: u64
}
trait Cost {
  fn retrieve_cost(&self) -> u64;
  fn retrieve_name(&self) -> String;
  fn retrieve_qty(&self) -> u64;
}
impl Cost for Carpet {
  fn retrieve_cost(&self) -> u64 {
    self.price * self.m2
  }
  fn retrieve_name(&self) -> String {
    self.name.clone()
  }
  fn retrieve_qty(&self) -> u64 {
    self.m2
  }
}
impl Cost for Tile {
  fn retrieve_cost(&self) -> u64 {
    self.price * self.m2
  }
  fn retrieve_name(&self) -> String {
    self.name.clone()
  }
  fn retrieve_qty(&self) -> u64 {
    self.m2
  }
}
impl Cost for Wood {
  fn retrieve_cost(&self) -> u64 {
    self.price * self.m2
  }
  fn retrieve_name(&self) -> String {
    self.name.clone()
  }
  fn retrieve_qty(&self) -> u64 {
    self.m2
  }
}
fn total_cost(list: Vec<Box<dyn Cost>>) -> u64 {
  let mut cost = 0;
  for i in list {
    cost = cost + i.retrieve_cost();
  }
  cost
}


fn my_main() {
  
  let persian_carpet = Carpet{name: String::from("Persian Carpet"),price:2000,m2:5}; // 10k
  let marmo_di_carrara = Tile{name: String::from("Marmo di Carrara"),price:500,m2:30}; // 15k
  let cameroon_ebony = Wood{name: String::from("Cameroon's Ebony"),price:2500,m2:5};    // 12.5k

  let material_list: Vec<Box<dyn Cost>> = vec![
    Box::new(marmo_di_carrara),
    Box::new(cameroon_ebony),
    Box::new(persian_carpet)
  ];

  for (index,item) in material_list.iter().enumerate() {
    println!("Cart's item N°{}\n{} m2 of {}\nCost:{}€\n", 
      index+1,
      item.retrieve_qty(),
      item.retrieve_name(),
      item.retrieve_cost()
    );
  }
  println!("Total cart's cost: {}€",  total_cost(material_list)); // 37.5k

}

//AI------------------------------------------------------------------------------------------------------------------------

fn ai_main () {}

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