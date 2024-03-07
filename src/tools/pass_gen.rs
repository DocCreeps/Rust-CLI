use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::*;
use rand::Rng;
use std::io;

fn generate_password(length: usize) -> String {
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL_CHARS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:'\",.<>?/";

    let mut rng = rand::thread_rng();
    let mut password_chars: Vec<char> = Vec::with_capacity(length);

    // Ajoutez au moins un caractère de chaque catégorie
    let dist = WeightedIndex::new(&[1; LOWERCASE.len()]).unwrap();
    password_chars.push(LOWERCASE[dist.sample(&mut rng)] as char);

    let dist = WeightedIndex::new(&[1; UPPERCASE.len()]).unwrap();
    password_chars.push(UPPERCASE[dist.sample(&mut rng)] as char);

    let dist = WeightedIndex::new(&[1; DIGITS.len()]).unwrap();
    password_chars.push(DIGITS[dist.sample(&mut rng)] as char);

    let dist = WeightedIndex::new(&[1; SPECIAL_CHARS.len()]).unwrap();
    password_chars.push(SPECIAL_CHARS[dist.sample(&mut rng)] as char);

    // Ajoutez le reste des caractères aléatoires
    for _ in 4..length {
        let category = rng.gen_range(0..4);
        let char_set = match category {
            0 => LOWERCASE,
            1 => UPPERCASE,
            2 => DIGITS,
            3 => SPECIAL_CHARS,
            _ => unreachable!(),
        };

        // Utilisez un vecteur à la place d'un tableau
        let weights: Vec<u32> = vec![1; char_set.len()];
        let dist = WeightedIndex::new(&weights).unwrap();
        let random_char = char_set[dist.sample(&mut rng)] as char;
        password_chars.push(random_char);
    }

    // Importez le trait SliceRandom pour utiliser la méthode shuffle
    password_chars.shuffle(&mut rng);

    let password: String = password_chars.into_iter().collect();
    password
}

pub fn main() {
    println!("Entrez la taille du mot de passe (entre 12 et 60) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
    let password_length: usize = input.trim().parse().expect("Veuillez entrer un nombre valide");

    if password_length < 12 || password_length > 60 {
        println!("La taille du mot de passe doit être entre 12 et 60 caractères.");
        return;
    }

    println!("Entrez le nombre de mots de passe à générer (max 10) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
    let num_passwords: usize = input.trim().parse().expect("Veuillez entrer un nombre valide");

    if num_passwords > 10 {
        println!("Le nombre de mots de passe ne peut pas dépasser 10.");
        return;
    }

    for _ in 0..num_passwords {
        let generated_password = generate_password(password_length);
        println!("Mot de passe généré : {}", generated_password);
    }
}
