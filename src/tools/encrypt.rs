use std::io;
use std::process::Command;


pub fn main() {
    //menu
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

fn cesar(){
    Command::new("clear").status().unwrap();
    println!("Vous avez choisi le cryptage Cesar");

    loop {
        let choice = menu();
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

fn vigenere(){
    Command::new("clear").status().unwrap();
    println!("Vous avez choisi le cryptage Vigenere");

    loop {
        let choice = menu();
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

fn menu() -> u32 {
    println!("1. Crypter");
    println!("2. Décrypter");
    println!("0. Retour au menu principal");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let choice: u32 = input.trim().parse().unwrap();
    return choice;

}

