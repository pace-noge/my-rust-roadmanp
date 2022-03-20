
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("Value of s is {}", s);

    // multiple time mutable reference

    let mut s1 = String::from("Hello");

    // let r1 = &mut s;               // first mutable reference
    // let r2 = &mut s;               // second reference

    println!("{}, {}", r1, r2);   // error -> mutable reference can be occured only once because there will be race condition possibility
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
