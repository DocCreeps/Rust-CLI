use std::io::{self, Write};

fn main() {
    // Initialisation des compteurs de victoires et défaites
    let mut victories = 0;
    let mut defeats = 0;

    // Boucle principale du jeu
    loop {
        // Génération d'un chiffre secret entre 0 et 1000
        let secret_number: u32 = rand::random::<u32>() % 1001;

        // Initialisation du compteur d'essais
        let mut attempts = 0;

        println!("Nouvelle partie !");

        // Boucle pour chaque tentative de devinette
        loop {
            // Vérification de la limite d'essais
            if attempts >= 10 {
                println!("Désolé, vous avez atteint la limite de {} essais. Le chiffre secret était : {}", attempts, secret_number);
                defeats += 1;
                break;
            }

            let mut guess = String::new();

            // Saisie de la proposition de l'utilisateur
            print!("Devinez le chiffre : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).expect("Erreur lors de la lecture de l'entrée.");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Veuillez entrer un nombre valide !");
                    continue;
                }
            };

            attempts += 1;

            // Vérification si la devinette est correcte
            if guess == secret_number {
                println!("Bravo ! Vous avez deviné le chiffre secret en {} essai(s).", attempts);
                victories += 1;
                break;
            } else if guess < secret_number {
                println!("Le chiffre secret est plus grand. Il vous reste {} essai(s).", 10 - attempts);
            } else {
                println!("Le chiffre secret est plus petit. Il vous reste {} essai(s).", 10 - attempts);
            }
        }

        // Affichage des statistiques de la session
        println!("Statistiques de la session - Victoires : {}, Défaites : {}", victories, defeats);

        let mut play_again = String::new();

        // Proposition de rejouer
        print!("Voulez-vous jouer à nouveau ? (O/N) : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut play_again).expect("Erreur lors de la lecture de l'entrée.");

        if play_again.trim().to_lowercase() != "o" {
            // Fin de la session si l'utilisateur ne souhaite pas rejouer
            println!("Merci d'avoir joué ! Fin de la session.");
            break;
        }
    }
}