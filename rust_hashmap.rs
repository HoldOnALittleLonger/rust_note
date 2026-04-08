use std::{collections::HashMap, string};

fn main() {
    let mut map: HashMap<u32, String> = HashMap::new();
    
    map.insert(0, String::from("zero"));
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    map.insert(3, String::from("three"));
    map.insert(4, String::from("four"));
    map.insert(5, String::from("five"));
    map.insert(6, String::from("six"));

    for (n, s) in &map {
        println!("debug @n - {n:?}, @s - {s:?}");
        println!("pair ({n}, {s})");
    }

    let e = map.entry(3);
    println!("Entry @e - {e:?}");

    let mref = map.entry(7).or_insert(String::from("seven"));
    println!("@mref - {mref:?}");

    for (n, s) in &map {
        println!("pair ({n}, {s})");
    }
}
