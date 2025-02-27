use std::collections::HashSet;

fn main() {
    let mut concert_que: HashSet<&str> = HashSet::new();
    println!("{concert_que:#?}");
    concert_que.insert("Reid");
    concert_que.insert("Rae");
    println!("{concert_que:#?}");
    println!("{:#?}", concert_que.len());

    concert_que.insert("Reid");
    println!("{concert_que:#?}");

    println!("{}", concert_que.remove("Rae"));
    println!("{}", concert_que.remove("Tol"));
    println!("{:#?}", concert_que);

    println!("{:#?}", concert_que.get("Reid").unwrap());
}
