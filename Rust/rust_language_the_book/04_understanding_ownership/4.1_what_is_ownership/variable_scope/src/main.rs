
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    
    // s1 already moved to s2
    // println!("{}, world", s1);
    println!("{}, world", s2);
}
