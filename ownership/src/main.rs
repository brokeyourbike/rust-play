fn main() {
    let reference_to_nothing = dangle();

    println!("{}", reference_to_nothing);

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
