use std::io;
use std::process::Command;
use rand::Rng;

pub fn main() {
    menu();
}
fn game(){
    let mut vie = 3;
    let mut nombretour = 0;
    let mut argentscore = 0;
    let mut bmaudite: i32;
    let mut argentboite: i32;
    let mut boiteselect: i32;

    println!("Bienvenue dans le jeu de la boite maudite, \nici tu n'as qu'un seul but, accumuler un maximum d'argent !\n\nPour cela c'est très simple, tu as devant toi trois boites, \ndeux d'entre elles contiennent de magnifiques pièces d'or, mais attention, la troisième contient un maléfice puissant qui te fait perdre une vie. \n\nTu disposes de trois vies et quand tu les as toutes perdues, la partie s'arrête!\n\nPour sélectioner une boite, rien de plus simple, il suffit d'entrer le numéro de la boite que tu souhaites ouvrir, mais attention la boite maudite change à chaque tour...\n\nTu peux organiser des challenges avec tes amis pour voir lequel a le plus de chance !\n\nBon courage, mais fait attention à la boite maudite !\n\n");

    loop {
        if vie == 0 {
            println!("Perdu, tu n'as plus de vie, ta course vers l'argent s'arrête ici, tu as gagné {} en {} coups.\nTu peux retenter ta chance, qui sais tu gagneras peut être des millions ! ", argentscore, nombretour);
            break;
        }
        nombretour += 1;

        let mut rng = rand::thread_rng();
        bmaudite = rng.gen_range(1..=3);
        argentboite = rng.gen_range(1..=100);

        loop {
            println!("Argent total : {}                                           \
            Vie : {} \n```````````````````````````````````````````````````````````````````\n              \
            Quelle boite choisis-tu ? (1, 2 ou 3)\n\n                            ___           ___           ___ \n                           | 1 |         | 2 |         | 3 |\n                            ```           ```           ```", argentscore, vie);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim().parse::<i32>() {
                Ok(num) => {
                    boiteselect = num;
                    Command::new("clear").status().unwrap();
                    break;
                }
                Err(_) => {
                    println!("Veuillez entrer un nombre valide.");
                }
            }
        }

        if boiteselect == bmaudite {
            vie -= 1;
            println!("C'est la boite maudite, il vous reste {} vie(s)", vie);
        } else if boiteselect >= 1 && boiteselect <= 3 {
            argentscore += argentboite;
            println!("Vous recevez {} pièces", argentboite);
        } else {
            vie -= 1;
            if nombretour == 0 {
                nombretour -= 1;
            }
            println!("Cette boite n'existe pas, tu perds une vie ! Fait attention la prochaine fois !");
        }
    }
}

fn menu(){
    loop {
        println!("Menu: ");
        println!("1. Jouer");
        println!("0. Retour au menu principal");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let opponent: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un choix valide (1 ou 0).");
                return;
            }
        };

        match opponent {
            1 => crate::games::boite_maudite::game(),
            0 => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Choix invalide"),
        }
    }
}