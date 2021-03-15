fn main() {
    let mut _s = String::new();

    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;
}
