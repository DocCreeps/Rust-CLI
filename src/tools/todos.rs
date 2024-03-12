use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    description: String,
    completed: bool,
}

pub fn main() {
    menu();
}

fn save_tasks(tasks: &Vec<Todo>) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&tasks)?;
    let mut file = File::create("todo_tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks() -> io::Result<Vec<Todo>> {
    let path = Path::new("todo_tasks.json");
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let tasks: Vec<Todo> = serde_json::from_reader(file)?;
    Ok(tasks)
}

fn add_task(tasks: &mut Vec<Todo>, title: String, description: String) {
    tasks.push(Todo {
        title,
        description,
        completed: false,
    });
}

fn modify_task(tasks: &mut Vec<Todo>, index: usize, title: Option<String>, description: Option<String>) {
    if let Some(task) = tasks.get_mut(index) {
        if let Some(title) = title {
            task.title = title;
        }
        if let Some(description) = description {
            task.description = description;
        }
    }
}

fn delete_task(tasks: &mut Vec<Todo>, index: usize) {
    tasks.remove(index);
}

fn toggle_task(tasks: &mut Vec<Todo>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = !task.completed;
    }
}

fn display_tasks(tasks: &Vec<Todo>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {} - {} - {} ", i + 1, task.title, if task.completed { "Compléter" } else { "En Attente" }, task.description);
    }
}



fn menu(){
    let mut tasks = load_tasks().unwrap_or_default();
    loop {
        println!("\nTODO Application\n");
        println!("1. Ajouter un tache");
        println!("2. Modifier une tache");
        println!("3. Supprimer une tache");
        println!("4. Basculer l'état d'une tache");
        println!("5. Listes des taches");
        println!("6. Sauvegarder et quitter");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Impossible de lire l'entrée utilisateur");

        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Titre de la tache:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Impossible de lire l'entrée utilisateur");

                println!("Description de la tache:");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Impossible de lire l'entrée utilisateur");

                add_task(&mut tasks, title.trim().to_string(), description.trim().to_string());
            }
            2 => {
                display_tasks(&tasks);

                println!("Indiquez le numéro de la tache à modifier:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");

                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => continue,
                };

                println!("Nouveau titre (Laisser blanc pour garder le même titre):");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Impossible de lire l'entrée utilisateur");

                println!("Nouvelle description (Laisser blanc pour garder le même titre) :");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Impossible de lire l'entrée utilisateur");

                modify_task(
                    &mut tasks,
                    index,
                    Some(title.trim().to_string()).filter(|_| !title.trim().is_empty()),
                    Some(description.trim().to_string()).filter(|_| !description.trim().is_empty()),
                );
            }
            3 => {
                display_tasks(&tasks);

                println!("Numéro de la tache à supprimer:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Impossible de lire l'entrée utilisateur");

                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => continue,
                };

                delete_task(&mut tasks, index);
            }
            4 => {
                display_tasks(&tasks);

                println!("Numéro de la tache à basculer:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Impossible de lire l'entrée utilisateur");

                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => continue,
                };

                toggle_task(&mut tasks, index);
            }
            5 => {
                display_tasks(&tasks);
            }
            6 => {
                save_tasks(&tasks).expect("Impossible de sauvegarder les taches");
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez entrer un numéro valide.");
            }
        }
    }
}