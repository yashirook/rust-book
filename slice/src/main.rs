fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);
    println!("word: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
