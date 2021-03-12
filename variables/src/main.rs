const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x + MAX_POINTS;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
