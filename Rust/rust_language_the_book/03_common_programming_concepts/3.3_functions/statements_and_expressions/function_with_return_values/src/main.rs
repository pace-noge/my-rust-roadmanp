
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("the value of x is: {}", x);

    println!("Plus one: {}", plus_one(x));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
