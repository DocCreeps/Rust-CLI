use std::io;
use std::process::Command;


pub fn main() {
    //menu
    menu_principal();
}

//fonction pour le cryptage cesar
fn cesar(){
    Command::new("clear").status().unwrap();
    println!("Vous avez choisi le cryptage Cesar");

    loop {
        let choice = menu_crypt();
        match choice {
            1 => crypter(),
            2 => decrypter(),
            0 => break,
            _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
        }
    }

    fn crypter(){
        println!("Vous avez choisi de crypter");
        println!("Entrez le texte à crypter :");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");

        print!("Entrez le décalage :");
        let mut shift = String::new();
        io::stdin().read_line(&mut shift).expect("Échec de la lecture de l'entrée");
        let shift: u8 = match shift.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Le décalage doit être un nombre entier");
                return;
            }
        };
        let output = caesar_helper(&input, shift);
        println!("Le texte chiffré est : {}", output);
        println!("Appuyez sur Entrée pour continuer...");
    }
    fn decrypter(){
        println!("Vous avez choisi de décrypter");
    }

}
//fonction pour le cryptage vigenere
fn vigenere(){
    Command::new("clear").status().unwrap();
    println!("Vous avez choisi le cryptage Vigenere");

    loop {
        let choice = menu_crypt();
        match choice {
            1 => crypter(),
            2 => decrypter(),
            0 => break,
            _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
        }
    }

    fn crypter(){
        println!("Vous avez choisi de crypter");
    }

    fn decrypter(){
        println!("Vous avez choisi de décrypter");
    }
}


//menu pour crypter ou décrypter
fn menu_crypt() -> u32 {
    println!("1. Crypter");
    println!("2. Décrypter");
    println!("0. Retour au menu principal");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let choice: u32 = input.trim().parse().unwrap();
    return choice;

}

//menu principal
fn menu_principal() {
    loop {
        Command::new("clear").status().unwrap();
        println!("Bienvenue dans l'outil de cryptage");
        println!("Choisissez votre méthode de cryptage :");
        println!("1. Cesar");
        println!("2. Vigenere");
        println!("0. Retour au menu principal");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");

        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => cesar(),
            2 => vigenere(),
            0 => break,
            _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
        }
    }
}

fn caesar_helper(input: &str, shift: u8) -> String {

    let mut output = String::new();
    for c in input.chars() {
        let shifted_char = if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            char::from(((c as u8 - base + shift) % 26) + base)
        } else {
            c
        };
        output.push(shifted_char);
    }
    output
}

