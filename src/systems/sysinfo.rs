use std::io;
use std::process::Command;
use whoami;
use ipconfig;
use sysinfo;
use std::env;



pub fn main(){
    menu();

}

fn all(){
    whoami_info();
}

fn whoami_info(){
    println!(
        "Nom réel utilisateur   whoami::realname():    {}",
        whoami::realname(),
    );
    println!(
        "Nom utilisateur        whoami::username():    {}",
        whoami::username(),
    );
    println!(
        "Languages               whoami::lang():        {:?}",
        whoami::lang().collect::<Vec<String>>(),
    );
    println!(
        "Nom ordinateur         whoami::devicename():  {}",
        whoami::devicename(),
    );
    println!(
        "Nom d'hote             whoami::hostname():    {}",
        whoami::hostname(),
    );
    println!(
        "OS                     whoami::platform():    {}",
        whoami::platform(),
    );
    println!(
        "OS Distro              whoami::distro():      {}",
        whoami::distro(),
    );
    println!(
        "Env.                   whoami::desktop_env(): {}",
        whoami::desktop_env(),
    );
    println!(
        "CPU Arch               whoami::arch():        {}",
        whoami::arch(),
    );
}





fn menu(){
    loop {
        loop {
            println!("Bienvenue dans l'outil info Système");
            println!("Choisissez une option :");
            println!("1. Tous voir");
            println!("2. Processeur");
            println!("3. CM");

            println!("0. Retour au menu principal");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");

            let choice: u32 = input.trim().parse().unwrap();

            match choice {

                1 => all(),
                0 => break,
                _ => println!("Choix invalide. Veuillez entrer un numéro valide."),
            }
        }
    }

}