use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{}", scores["Blue"]);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);

    // invalid usage
    // println!("{}", field_name);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{} - {:#?}", team_name, score);

    // iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update hashmap
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores.get(&team_name));

    // update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
