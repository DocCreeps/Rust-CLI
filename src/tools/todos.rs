use std::io;
use std::process::Command;
struct Todo {
    title: String,
    description: String,
    completed: bool,
}

pub fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("1. Ajouter une tâche");
        println!("2. Modifier une tâche");
        println!("3. Valider/devalider une tâche");
        println!("4. Afficher les tâches");
        println!("0. Quitter");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim().parse::<usize>().unwrap();

        match choice {
            1 => add_task(&mut tasks),
            2 => modify_task(&mut tasks),
            3 => toggle_task(&mut tasks),
            4 => display_tasks(&tasks),
            0 => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Choix invalide"),
        }
    }
    Command::new("clear").status().unwrap();
}

fn add_task(tasks: &mut Vec<Todo>){
    println!("Entrez le titre de la tâche :");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    println!("Entrez la description de la tâche :");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    let task = Todo {
        title,
        description,
        completed: false,
    };

    tasks.push(task);
}

fn modify_task(tasks: &mut Vec<Todo>) {
    println!("Entrez l'index de la tâche à modifier :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        println!("Index invalide");
        println!("Voulez vous crée cette tâche ? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            add_task(tasks);
            return;
        }
        //back menu
        return;
    }

    println!("Entrez le nouveau titre de la tâche :");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    println!("Entrez la nouvelle description de la tâche :");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    tasks[index] = Todo {
        title,
        description,
        completed: tasks[index].completed,
    };
}

fn toggle_task(tasks: &mut Vec<Todo>){
    println!("Entrez l'index de la tâche à basculer :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        println!("Cette index n'existe pas");
        return;
    }

    tasks[index].completed = !tasks[index].completed;

}

fn display_tasks(tasks: &Vec<Todo>) {
    println!("Liste des tâches :");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {} {}", i, task.title, if task.completed { "Complétée" } else { "En attente" });
        println!();
    }
}