# Rust CLI

Différent programme CLI fait en rust 

Un jeu simple en Rust où l'utilisateur doit deviner un nombre secret généré aléatoirement.

Un jeu de rock paper scissor. 

## Table des matières
- [Installation](#installation)
- [Exécution](#exécution)
- [Fonctionnalités](#fonctionnalités)
- [Games](#games)
  - [Secret_Number](#secret_number)
  - [Rock_Paper_Scissor](#rock_paper_scissor)
  - [Tic_Tac_Toe](#tic_tac_toe)
- [Tools](#tools)
    - [Todo](#todo)
    - [Password_Generator](#password_generator)
- [Configuration](#configuration)

## Installation

Assurez-vous d'avoir [Rust](https://www.rust-lang.org/) installé sur votre système. Clonez le dépôt vers votre machine locale :

```
git clone https://github.com/DocCreeps/Rust-CLI.git

cd Rust-CLI
```

## Exécution
Utilisez Cargo pour construire et exécuter le jeu :

```
cargo run
```

## Fonctionnalités

### Games

#### Secret_Number
Génération aléatoire du chiffre secret : À chaque nouvelle partie, un chiffre secret est généré de manière aléatoire entre 0 et 1000.  
Limite de 10 essais par partie : Le joueur a jusqu'à 10 essais pour deviner le chiffre secret.  
Statistiques de la session : Les victoires, défaites, et la moyenne d'essais sont affichées à la fin de chaque session.  
Liste des essais réussis : Une liste détaillée du nombre d'essais et du nombre de victoires est affichée.  

#### Rock_Paper_Scissor
Le joueur fait un choix parmis ses 3 : Pierre, Papier, Ciseaux 
L'ordi a un choix aléatoire
La pierre bat le ciseaux
Le ciseaux bat le papier
Le papier bat la pierre

#### Tic_Tac_Toe
Un jeu de morpion

### Tools

#### Todo
Permet de créer une liste de tâches à faire, à faire, en cours et faites.

#### Password_Generator
Permet de générer un ou plusieur mot de passe aléatoire de la longueur souhaitée.


## Configuration
Aucune configuration spécifique n'est requise pour exécuter le jeu. Cependant, le fichier Cargo.toml contient des dépendances nécessaires au bon fonctionnement du projet.
