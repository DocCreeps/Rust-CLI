use std::io;
mod games {
    pub mod secret_number;
    pub mod rock_paper_scissors;
}
fn main() {
    loop {
        println!("Menu:");
        println!("1. Programme Secret Number");
        println!("2. Programme Rock Paper Scissors");
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