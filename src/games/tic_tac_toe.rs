use std::io;

pub fn main() {
    let mut board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut current_player = 'X';

    loop {
        print_board(&board);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Impossible de lire l'entrée utilisateur.");

        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if coords.len() != 2 || coords[0] > 2 || coords[1] > 2 {
            println!("Coordonnées invalides, veuillez réessayer.");
            continue;
        }

        if board[coords[0]][coords[1]] != ' ' {
            println!("Mouvement invalide, veuillez réessayer.");
            continue;
        }

        board[coords[0]][coords[1]] = current_player;

        if check_win(&board, current_player) {
            print_board(&board);
            println!("Joueurs {} a gagné!", current_player);
            break;
        } else if check_draw(&board) {
            print_board(&board);
            println!("Match nul!");
            break;
        } else {
            current_player = if current_player == 'X' { 'O' } else { 'X' };
        }
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    println!("Choisir une coordonnée valide sous la forme 'ligne colonne' (0-2) :");
    println!("  0   1   2");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i);
        for col in row {
            print!("{} | ", col);
        }
        println!("\n----+---+---+");
    }
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    for row in board {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }

    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }

    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }

    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

fn check_draw(board: &[[char; 3]; 3]) -> bool {
    for row in board {
        for col in row {
            if *col == ' ' {
                return false;
            }
        }
    }

    true
}
