fn main() {
    let _x = five();

    let y = {
        let x = plus_one(3);
        x
    };

    another_function(y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
