# Rust CLI

Différent programme CLI fait en rust 

Un jeu simple en Rust où l'utilisateur doit deviner un nombre secret généré aléatoirement.

Un jeu de rock paper scissor. 

## Table des matières
- [Installation](#installation)
- [Exécution](#exécution)
- [Fonctionnalités](#fonctionnalités)
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
 
### Menu
un menu pour faire le choix du programme a lancer ou pour quitter

### Secret_Number 
Génération aléatoire du chiffre secret : À chaque nouvelle partie, un chiffre secret est généré de manière aléatoire entre 0 et 1000.  
Limite de 10 essais par partie : Le joueur a jusqu'à 10 essais pour deviner le chiffre secret.  
Statistiques de la session : Les victoires, défaites, et la moyenne d'essais sont affichées à la fin de chaque session.  
Liste des essais réussis : Une liste détaillée du nombre d'essais et du nombre de victoires est affichée.  

### Rock_Paper_Scissor
Le joueur fait un choix parmis ses 3 : Pierre, Papier, Ciseaux 
L'ordi a un choix aléatoire
La pierre bat le ciseaux
Le ciseaux bat le papier
Le papier bat la pierre


## Configuration
Aucune configuration spécifique n'est requise pour exécuter le jeu. Cependant, le fichier Cargo.toml contient des dépendances nécessaires au bon fonctionnement du projet.
