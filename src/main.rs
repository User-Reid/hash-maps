use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert("Salmon".to_string(), 32.99);
    menu.insert("Steak".to_string(), 42.99);
    menu.insert("Lobster".to_string(), 32.99);

    println!("{menu:#?}");

    // let mut country_capital: HashMap<&str, &str> = HashMap::new();
    let mut country_capital = HashMap::<&str, &str>::new();

    country_capital.insert("France", "Paris");
    country_capital.insert("Germany", "Berlin");
    country_capital.insert("Austin", "Texas");

    println!("{country_capital:#?}");
}
