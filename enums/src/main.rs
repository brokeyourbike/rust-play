#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    println!("value of the coin is {}", Coin::Nickel.value_in_cents());
    println!("value of the coin is {}", Coin::Quarter(UsState::Alaska).value_in_cents());
    println!("value of the coin is {}", Coin::Quarter(UsState::Alabama).value_in_cents());
}
