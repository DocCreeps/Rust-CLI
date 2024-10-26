use std::io;
use whoami;
use chrono;
use std::process::Command;

mod games {
    pub mod secret_number;
    pub mod rock_paper_scissors;
    pub mod tic_tac_toe;
    pub mod boite_maudite;
    pub mod batonnets;
    pub mod game421;
    pub mod pendu;
}

mod tools {
    pub mod todos;
    pub mod pass_gen;
    pub mod timer;
    pub mod encrypt;
pub mod temperatureconverter;
}
mod systems {
    pub mod sysinfo;
    pub mod benchmark;
    
    pub mod algoshort;
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
        println!("3. Système");
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
                    println!("4. Boite Maudite");
                    println!("5. Bâtonnets");
                    println!("6. 421 ");
                    println!("7. Pendu");
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
                            println!("Vous avez choisi le jeu Secret Number");
                            games::secret_number::secret_number_game();
                        }
                        2 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu Rock Paper Scissors");
                            games::rock_paper_scissors::main();
                        }
                        3 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu Tic Tac Toe");
                            games::tic_tac_toe::main();
                        }
                        4 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu Boite Maudite");
                            games::boite_maudite::main();
                        }
                        5 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu Bâtonnets");
                            games::batonnets::main();
                        }
                        6 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu 421");
                            games::game421::main();
                        }
                        7 =>{
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le jeu Pendu");
                            games::pendu::main();
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
                    println!("5. Temperature Converter");
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
                        5 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Convertisseur Temperature");
                            tools::temperatureconverter::main();
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
                    println!("2. Benchmark");
                    println!("3. Algo Short");
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
                        2 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Benchmark");
                            systems::benchmark::main();
                        }
                        3 => {
                            Command::new("clear").status().unwrap();
                            println!("Vous avez choisi le Programme Algo Short");
                            systems::algoshort::main();
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
