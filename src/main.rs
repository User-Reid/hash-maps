use std::collections::HashMap;

fn main() {
    let data: [(&str, u8); 4] = [("Brandon", 2), ("Cameron", 7), ("Reid", 3), ("Rachel", 4)];
    let mut x: HashMap<&str, u8> = HashMap::from(data);
    println!("{x:#?}");

    let reid: Option<u8> = x.remove("Reid");
    println!("{:#?}", reid.unwrap());
    let reid: Option<u8> = x.remove("Reid");
    println!("{:#?}", reid);



    println!("{x:#?}");
}
