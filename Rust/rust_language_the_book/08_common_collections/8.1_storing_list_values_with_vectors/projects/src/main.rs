#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exists = &v[100];
    // println!("{}", does_not_exists);
    // let mut v1 = vec![1, 2, 3, 4, 5];
    // let first = &v1[0];  // borrow occurs here
    // v1.push(6);  // new element mean, new memory allocation for entire vector
    // println!("The first element is: {}", first); // already on different memory

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for r in &row {
        println!("{:?}", r);
    }

    let vs = vec!["halo", "world"];
    for s in &vs {
        println!("{}", s);
    }


}
