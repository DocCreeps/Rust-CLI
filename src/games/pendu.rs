use rand::prelude::*;
use std::collections::HashSet;
use std::io::{self, Write};

const MAX_TRIES: u32 = 6;
const WORDS: &[&str] = &["rust", "programming", "computer", "algorithm", "developer"];

pub fn main() {
    let mut rng = rand::thread_rng();
    let word = WORDS[rng.gen_range(0..WORDS.len())].to_string();
    let mut guessed_letters = HashSet::new();
    let mut tries_left = MAX_TRIES;

    println!("Bienvenue au jeu du Pendu !");

    println!("Le mot a {} lettres.", word.len());

    while tries_left > 0 {
        print_word_progress(&word, &guessed_letters);
        print_hangman(tries_left);
        println!("Il vous reste {} essais.", tries_left);

        print!("Devinez une lettre : ");
        io::stdout().flush().unwrap();

        let guess = get_user_input().to_lowercase();
        if guess.len() != 1 {
            println!("Veuillez entrer une seule lettre.");
            continue;
        }

        let guess_char = guess.chars().next().unwrap();
        if !guess_char.is_ascii_alphabetic() {
            println!("Veuillez entrer une lettre valide.");
            continue;
        }

        if guessed_letters.contains(&guess_char) {
            println!("Vous avez déjà deviné cette lettre.");
            continue;
        }

        guessed_letters.insert(guess_char);
        if !word.contains(guess_char) {
            tries_left -= 1;
            println!("Incorrect !");

        } else {
            println!("Correct !");
        }

        if word.chars().all(|c| guessed_letters.contains(&c)) {
            println!("Félicitations ! Vous avez deviné le mot : {}", word);
            return;
        }
    }

    println!("Désolé, vous avez épuisé tous vos essais. Le mot était : {}", word);
}

fn print_word_progress(word: &str, guessed_letters: &HashSet<char>) {
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            print!("{} ", c);
        } else {
            print!("_ ");
        }
    }
    println!();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn print_hangman(tries_left: u32) {
    match tries_left {
        6 => println!("+---+
            |
            |
            |
            ==="),
        5 => println!("+---+
            |   O
            |
            |
            ==="),
        4 => println!("+---+
            |   O
            |   |
            |
            ==="),
        3 => println!("+---+
            |   O
            |  /|
            |
            ==="),
        2 => println!("+---+
            |   O
            |  /|\\
            |
            ==="),
        1 => println!("+---+
            |   O
            |  /|\\
            |  /
            ==="),
        0 => println!("+---+
            |   O
            |  /|\\
            |  / \\
            ==="),
        _ => (),
    }
}
