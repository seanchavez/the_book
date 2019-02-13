fn main() {
    let mut s = String::from("hello world");
    let word = first_word_a(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word_a(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}