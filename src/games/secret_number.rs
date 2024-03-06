use std::io::{self, Write};
use std::collections::HashMap;

pub fn secret_number_game() {
    // Initialisation des variables de statistiques
    let mut victories = 0;
    let mut defeats = 0;
    let mut total_attempts = 0;
    let mut attempts_counter: HashMap<u32, u32> = HashMap::new();

    // Boucle principale du jeu
    loop {
        // Génération d'un nouveau nombre secret entre 0 et 1000
        let secret_number: u32 = rand::random::<u32>() % 1001;
        let mut attempts = 0;

        println!("Nouvelle partie !");

        // Boucle du jeu
        loop {
            // Vérification du nombre d'essais
            if attempts >= 10 {
                println!("Désolé, vous avez atteint la limite de {} essais. Le chiffre secret était : {}", attempts, secret_number);
                defeats += 1;
                break;
            }

            // Lecture de l'entrée utilisateur
            let mut guess = String::new();
            print!("Devinez le chiffre : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).expect("Erreur lors de la lecture de l'entrée.");

            // Conversion de l'entrée en u32
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Veuillez entrer un nombre valide !");
                    continue;
                }
            };

            // Incrémentation du nombre d'essais
            attempts += 1;
            total_attempts += 1;

            // Vérification du guess par rapport au nombre secret
            if guess == secret_number {
                println!("Bravo ! Vous avez deviné le chiffre secret en {} essai(s).", attempts);
                victories += 1;
                *attempts_counter.entry(attempts).or_insert(0) += 1;
                break;
            } else if guess < secret_number {
                println!("Le chiffre secret est plus grand. Il vous reste {} essai(s).", 10 - attempts);
            } else {
                println!("Le chiffre secret est plus petit. Il vous reste {} essai(s).", 10 - attempts);
            }
        }

        // Affichage des statistiques de la session
        println!("Statistiques de la session - Victoires : {}, Défaites : {}, Moyenne d'essais : {:.2}", victories, defeats, total_attempts as f32 / (victories + defeats) as f32);

        // Affichage de la liste du nombre d'essais et du nombre de victoires
        println!("Liste des essais réussis :");
        for attempts in 1..=10 {
            let wins = attempts_counter.get(&attempts).unwrap_or(&0);
            println!(" {} essai(s) : {} victoire(s)", attempts, wins);
        }

        // Demande à l'utilisateur s'il souhaite jouer à nouveau
        let mut play_again = String::new();
        print!("Voulez-vous jouer à nouveau ? (O/N) : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut play_again).expect("Erreur lors de la lecture de l'entrée.");

        if play_again.trim().to_lowercase() != "o" {
            println!("Merci d'avoir joué ! Fin de la session.");
            break;
        }
    }
}

