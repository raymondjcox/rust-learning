extern crate rand;
use std::io;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Welcome to hangman!");
    let words = ["Jiayi", "RJ"];
    let chosen_word = words[rand::thread_rng().gen_range(0, words.len())];
    let mut guesses_left = chosen_word.chars().count() + 2;
    println!("You have a total of {} guesses. The word is {} characters long", guesses_left, chosen_word.chars().count());
    let mut guesses = HashMap::new();

    while guesses_left > 0 {
        println!("What's your guess?");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: char = match guess.trim().parse() {
            Ok(ch) => ch,
            Err(_) => {
                println!("You may only enter 1 letter at a time");
                continue
            },
        };
        guesses.insert(guess, true);

        let mut guessed_word = String::new();
        for c in chosen_word.chars() {
            match guesses.get(&c) {
                Some(_) => guessed_word.push(c),
                None => guessed_word.push('_'),
            };
        }
        println!("Guessed word: {}", guessed_word);

        guesses_left -= 1;
        if guessed_word == chosen_word {
            println!("You win! with {} guesses left", guesses_left);
            return;
        }
        println!("You have {} guesses left", guesses_left);
    }

    println!("You lose!");
}
