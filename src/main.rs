use std::io;
use std::collections::HashMap;
// this is a titanium moment


fn main() {
    let values = HashMap::from([
        ('b', "110"),
        ('x', "105"),
        ('i', "105"),
        ('r', "105"),
        ('t', "100"),
        ('a', "100"),
        ('k', "100"),
        ('s', "100"),
        ('e', "100"),
        ('m', "95"),
        ('z', "93"),
        ('v', "93"),
        ('h', "92"),
        ('d', "92"),
        ('f', "92"),
        ('n', "91"),
        ('y', "90"),
        ('u', "85"),
        ('w', "82"),
        ('o', "82"),
        ('p', "76"),
        ('c', "74"),
        ('g', "50"),
        ('l', "30"),
        ('q', "30"),
        ('j', "-12"),
        ]);
    let mut value: f32 = 0.0;

    let mut input = String::new();

    println!("Enter string to be evaluated");

    io::stdin().read_line(&mut input).unwrap();

    input = input.to_lowercase();

    for c in input.chars() { 

        match values.get(&c) {
            Some(&number) => value += number.parse::<f32>().unwrap(),
            _ => print!(""),
        };

    };
    //println!("{}",&value);

    println!("{}",&value/input.replace("\r\n", "").replace("\r", "").replace("\n", "").chars().count().to_string().parse::<f32>().unwrap()); // i know your gonna love this


}

