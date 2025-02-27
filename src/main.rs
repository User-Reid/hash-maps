use std::collections::HashSet;

fn main() {
    let mut concert_que: HashSet<&str> = HashSet::new(); // Reid, Rae
    let mut movie_que: HashSet<&str> = HashSet::new(); // Reid, Troy

    concert_que.insert("Reid");
    concert_que.insert("Rae");

    movie_que.insert("Reid");
    movie_que.insert("Troy");

    println!("{:#?}", concert_que.union(&movie_que));
    println!("{:#?}", concert_que.difference(&movie_que));
    println!("{:#?}", movie_que.difference(&concert_que));

    println!("{:#?}", movie_que.symmetric_difference(&concert_que));

    println!("{:#?}", movie_que.is_disjoint(&concert_que));

    println!("{:#?}", movie_que.is_subset(&concert_que));

    let mut attendies: HashSet<&str> = HashSet::new();
    attendies.insert("Reid");
    println!("{:#?}", concert_que.is_subset(&attendies));
}
