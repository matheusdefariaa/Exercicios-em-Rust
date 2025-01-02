use std::io;
fn main() {
    let mut word: String = String::new();

    println!("Enter the word");

    io::stdin()
    .read_line(&mut word)
    .expect("Failed to read line");

    let word: String = word.trim().parse().expect("Err");

    let to_check: bool = is_palindrome(&word);

    println!("Is Palindrome: {to_check}")
}

fn is_palindrome(word: &String) -> bool {

    if &word.chars().rev().collect::<String>() == word {
        return true;
    }

    false
}
