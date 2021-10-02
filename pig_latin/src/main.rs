
fn main() {
    let vovels = vec!["a", "e", "i", "o", "u"];

    let mut original: Vec<String> = vec![String::from("apple"), String::from("first")];

    println!("Original values: {:?}", original);

    for word in &mut original {
        let first_letter = &word[..1];

        if vovels.contains(&first_letter) {
            word.push_str("-hay");
        } else {
            *word = format!("{}-{}ay", &word[1..], first_letter);
        }
    }

    println!("Updated values: {:?}", original);
}
