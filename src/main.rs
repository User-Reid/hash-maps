use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink: String = String::from("Latte");
    let milk: String = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    let value: &str = coffee_pairings
        .get("Flat White")
        .copied()
        .unwrap_or("Shit aint there cousin");
    println!("{}", value);
    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{coffee_pairings:#?}")
}
