use std::io;
use whoami;
use chrono;
use std::process::Command;

mod games {
    pub mod secret_number;
    pub mod rock_paper_scissors;
    pub mod tic_tac_toe;
}

mod tools {
    pub mod todos;
    pub mod pass_gen;
    pub mod timer;
    pub mod encrypt;
}
mod systems {
    pub mod sysinfo;
}



fn main() {
    let date: String = chrono::Local::now().format("%d %b %Y").to_string();
    let time: String = chrono::Local::now().format("%H:%M:%S").to_string();
    let user: String = whoami::username();

    loop {
        println!("Bienvenue {} ", user);
        println!("Nous sommes le : {} il est {}", date, time);
        println!("Menu principal:");
        println!("1. Jeux");
        println!("2. Outils");
        println!("3. Systeme");
        println!("0. Quitter");

        let mut main_choice = String::new();

        io::stdin().read_line(&mut main_choice).expect("Erreur lors de la lecture de la ligne");

        let main_choice: u32 = match main_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match main_choice {
            1 => {
                // Menu des jeux
                loop {
                    println!("Menu Jeux:");
                    println!("1. Secret Number");
                    println!("2. Rock Paper Scissors");
                    println!("3. Tic Tac Toe");
                    println!("0. Retour au menu principal");

                    let mut game_choice = String::new();
                    io::stdin().read_line(&mut game_choice).expect("Erreur lors de la lecture de la ligne");

                    let game_choice: u32 = match game_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    match game_choice {
                        1 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Secret Number");
                            games::secret_number::secret_number_game();
                        }
                        2 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Rock Paper Scissors");
                            games::rock_paper_scissors::main();
                        }
                        3 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Tic Tac Toe");
                            games::tic_tac_toe::main();
                        }
                        0 => break,
                        _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
                    }
                }
            }
            2 => {
                // Menu des outils
                loop {
                    println!("Menu Outils:");
                    println!("1. Todo List");
                    println!("2. Password Generator");
                    println!("3. Timer");
                    println!("4. Encrypt/Decrypt (Coming soon)");
                    println!("0. Retour au menu principal");

                    let mut tool_choice = String::new();
                    io::stdin().read_line(&mut tool_choice).expect("Erreur lors de la lecture de la ligne");

                    let tool_choice: u32 = match tool_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    match tool_choice {
                        1 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Todo List");
                            tools::todos::main();
                        }
                        2 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Password Generator");
                            tools::pass_gen::main();
                        }
                        3 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Timer");
                            tools::timer::main();
                        }
                        4 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Encrypt/Decrypt");
                            tools::encrypt::main();
                        }
                        0 => break,
                        _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
                    }
                }
            }
            3 => {
                // Menu des outils systèmes
                loop {
                    println!("Menu des outils systèmes:");
                    println!("1. Info Système");
                    println!("0. Retour au menu principal");

                    let mut system_choice = String::new();
                    io::stdin().read_line(&mut system_choice).expect("Erreur lors de la lecture de la ligne");

                    let system_choice: u32 = match system_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    match system_choice {
                        1 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Info système");
                            systems::sysinfo::main();
                        }
                        0 => break,
                        _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
                    }
                }
            }
            0 => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
        }
    }
}
