use std::io;
use whoami;
use chrono;

mod games {
    pub mod secret_number;
    pub mod rock_paper_scissors;
}

fn main() {
    // Obtention de la date local et formatage
    let date: String = chrono::Local::now().format("%d %b %Y").to_string();

// Obtention de l'heure local et formatage
    let time: String = chrono::Local::now().format("%H:%M:%S").to_string();

// Obtention du nom d'utilisateur
    let user: String = whoami::username();


    loop {
        println!("Bienvenue {} ", user);
        println!("Nous somme le : {} il est {}", date, time);
        println!("Menu:");
        println!("1. Secret Number");
        println!("2. Rock Paper Scissors");
        // Ajoutez d'autres options de menu pour d'autres programmes

        println!("0. Quitter");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Erreur lors de la lecture de la ligne");

        // Convertissez le choix en nombre entier
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Vous avez choisi le Programme Secret Number");
                games::secret_number::secret_number_game();
            }
            2 => {
                println!("Vous avez choisi le Programme Rock Paper Scissors");
                games::rock_paper_scissors::main();
            }


            0 => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
        }
    }
}