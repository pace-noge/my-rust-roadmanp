

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) {
    match coin {
        coin::Penny 
    }
}
