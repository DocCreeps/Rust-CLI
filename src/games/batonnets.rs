use std::io;

pub fn main() {
    println!("Bienvenue dans le jeu des bâtonnets !");
    menu();
}

fn get_valid_input(sticks: u32) -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let sticks_to_remove: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide entre 1 et 3.");
                continue;
            }
        };

        if sticks_to_remove < 1 || sticks_to_remove > 3 {
            println!("Veuillez entrer un nombre valide entre 1 et 3.");
            continue;
        }

        if sticks_to_remove > sticks {
            println!("Désolé, vous ne pouvez pas prendre plus de bâtonnets qu'il n'y en a.");
            continue;
        }

        return sticks_to_remove;
    }
}

fn human_vs_human() {
    let mut sticks = 21;
    let mut score_j1 =0;
    let mut score_j2 =0;

    println!("Il y a actuellement {} bâtonnets.", sticks);

    loop {
        let sticks_to_remove = get_valid_input(sticks);

        sticks -= sticks_to_remove;
        println!("Vous avez pris {} bâtonnets.\nIl reste {} bâtonnets", sticks_to_remove, sticks);

        if sticks == 0 {
            println!("Désolé, vous avez perdu ! Vous avez pris le dernier bâtonnet.");
            score_j2 +=1;
            break;
        }

        println!("C'est au tour du joueur 2 de jouer.");
        let sticks_to_remove = get_valid_input(sticks);

        sticks -= sticks_to_remove;
        println!("Le joueur 2 a pris {} bâtonnets.\nIl reste {} bâtonnets", sticks_to_remove, sticks);

        if sticks == 0 {
            println!("Félicitations, vous avez gagné ! Le joueur 2 a pris le dernier bâtonnet.");
            score_j1 +=1;
            break;
        }
        println!("C'est au tour du joueur 1 de jouer.");
    }
    println!("J1 : {} VS J2 : {}", score_j1, score_j2);
}


fn human_vs_ai(){
    let mut sticks = 21;
    let mut score_joueur = 0;
    let mut score_ai = 0;

    println!("Il y a actuellement {} bâtonnets.", sticks);

    loop {
        let sticks_to_remove = get_valid_input(sticks);

        sticks -= sticks_to_remove;
        println!("Vous avez pris {} bâtonnets.\nIl reste {} bâtonnets", sticks_to_remove, sticks);

        if sticks == 0 {
            println!("Désolé, vous avez perdu ! Vous avez pris le dernier bâtonnet.");
            score_ai +=1;
            break;
        }

        let mut ai_sticks_to_remove = sticks % 4;
        if ai_sticks_to_remove == 0 {
            ai_sticks_to_remove = 1;
        }

        sticks -= ai_sticks_to_remove;
        println!("L'ordinateur a pris {} bâtonnets.\nIl reste {} bâtonnets", ai_sticks_to_remove, sticks);

        if sticks == 0 {
            println!("Félicitations, vous avez gagné ! L'ordinateur a pris le dernier bâtonnet.");
            score_joueur += 1;
            break;
        }

    }
    println!("Joueur : {} VS AI : {}", score_joueur, score_ai);
}

fn menu(){
    loop {
    println!("Choisissez votre adversaire : ");
    println!("1. Humain");
    println!("2. Ordinateur");
    println!("0. Retour au menu principal");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let opponent: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un choix valide (1 ou 2).");
            return;
        }
    };

    match opponent {
        1 => human_vs_human(),
        2 => human_vs_ai(),
        0 => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Choix invalide"),
        }
    }
}