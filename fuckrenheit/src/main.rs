use std::io;

fn main() {
    println!("Convert celcius to fuckrenheit!");

    loop {
        println!("Please input your temperature in celsius:");

        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("We trying to fuckrenheit, do not mess your damn input!");

        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("What the heck are you typing!??");
                continue
            },
        };

        let fuckrenheit = celsius * 1.8 + 32.0;

        break println!("Fuckrenheit value: {}", fuckrenheit);
    }
}
