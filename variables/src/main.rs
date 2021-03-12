const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x + MAX_POINTS;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    let (_, _, z) = tup;
    println!("The value of z is: {}", z);
}
