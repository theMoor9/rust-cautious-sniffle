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
// Step 1: Definire un Trait per ottenere il costo per metro quadro
trait MaterialAi {
  fn cost_per_square_meter_ai(&self) -> f64;
}

// Step 2: Creare le strutture per i diversi materiali
struct CarpetAi;
struct TileAi;
struct WoodAi;

// Step 2: Implementare il Trait per ogni materiale
impl MaterialAi for CarpetAi {
  fn cost_per_square_meter_ai(&self) -> f64 {
    2000.0
  }
}

impl MaterialAi for TileAi {
  fn cost_per_square_meter_ai(&self) -> f64 {
      500.0
  }
}

impl MaterialAi for WoodAi {
  fn cost_per_square_meter_ai(&self) -> f64 {
    2500.0
  }
}

// Step 3: Creare una funzione per calcolare il costo totale
fn calculate_total_cost_ai(materials_ai: Vec<(&dyn MaterialAi, f64)>) -> f64 {
  let mut total_cost_ai = 0.0;

  for (material_ai, area_ai) in materials_ai {
      total_cost_ai += material_ai.cost_per_square_meter_ai() * area_ai;
  }

  total_cost_ai
}

fn ai_main () {
  // Crea le istanze dei materiali
  let carpet_ai = CarpetAi;
  let tile_ai = TileAi;
  let wood_ai = WoodAi;

  // Definisci una lista di materiali con i metri quadri richiesti
  let materials_ai: Vec<(&dyn MaterialAi, f64)> = vec![
      (&carpet_ai, 5.0), // 50 metri quadri di Carpet
      (&tile_ai, 30.0),   // 30 metri quadri di Tile
      (&wood_ai, 5.0),   // 20 metri quadri di Wood
  ];

  // Calcola il costo totale
  let total_cost_ai = calculate_total_cost_ai(materials_ai);

  println!("The total cost of materials is: ${}", total_cost_ai);
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