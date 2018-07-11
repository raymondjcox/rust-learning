use std::io;

fn main() {
    println!("Enter a word.");
    let mut word = String::new();
    io::stdin().read_line(&mut word)
        .expect("Failed to read line");
    word.pop();

    let mut new_word = String::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&first_char) {
        new_word.push_str(&word);
        new_word.push_str("-hay");
    } else {
        for letter in chars {
            new_word.push(letter);
        }

        new_word.push('-');
        new_word.push(first_char);
        new_word.push_str("ay");
    }

    println!("{}", new_word);
}
