use std::io;

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Pierre,
    Papier,
    Ciseaux,
}

struct GameStats {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl GameStats {
    fn new() -> Self {
        GameStats {
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn display(&self) {
        println!("Gagné: {}, Nul: {}, Perdu: {}", self.wins, self.draws, self.losses);
    }
}

fn get_user_choice() -> Choice {
    loop {
        println!("Votre choix (Pierre, Papier, Ciseaux):");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Impossible de lire l'entrée.");

        match user_input.trim().to_lowercase().as_str() {
            "pierre" => return Choice::Pierre,
            "papier" => return Choice::Papier,
            "ciseaux" => return Choice::Ciseaux,
            _ => println!("Choix invalide. Veuillez entrer un choix valide."),
        }
    }
}

fn get_computer_choice() -> Choice {
    use rand::Rng;
    let choices = [Choice::Pierre, Choice::Papier, Choice::Ciseaux];
    let random_index = rand::thread_rng().gen_range(0..choices.len());
    choices[random_index].clone()
}

fn determine_winner(user_choice: Choice, computer_choice: Choice) -> &'static str {
    if user_choice == computer_choice {
        "Match nul!"
    } else if (user_choice == Choice::Pierre && computer_choice == Choice::Ciseaux)
        || (user_choice == Choice::Papier && computer_choice == Choice::Pierre)
        || (user_choice == Choice::Ciseaux && computer_choice == Choice::Papier)
    {
        "Vous avez gagné !"
    } else {
        "L'ordi a gagné !"
    }
}

fn rock_paper_scissors_game(stats: &mut GameStats) {
    let user_choice = get_user_choice();
    let computer_choice = get_computer_choice();

    println!("Votre choix: {:?}", user_choice);
    println!("Choix de l'ordi: {:?}", computer_choice);

    let result = determine_winner(user_choice, computer_choice);
    println!("{}", result);

    match result {
        "Match nul!" => stats.draws += 1,
        "Vous avez gagné !" => stats.wins += 1,
        "L'ordi a gagné !" => stats.losses += 1,
        _ => (),
    }

    stats.display();
}

pub(crate) fn main() {
    println!("Bienvenue dans le jeu Pierre, Papier, Ciseaux");

    let mut stats = GameStats::new();

    loop {
        rock_paper_scissors_game(&mut stats);

        println!("Voulez-vous rejouer ? (O/N)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Impossible de lire l'entrée.");

        if play_again.trim().to_lowercase() != "o" {
            println!("Merci d'avoir joué !");
            println!("Au revoir !");
            break;
        }
    }
}
