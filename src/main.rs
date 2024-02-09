// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn main() {
    let str = String::from("Hello, World!");
    let word = first_word(&str);
    println!("The first word is {}", word);
    let word = first_word(&str[0..10]);
    println!("The first word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
