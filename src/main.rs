use std::io::{self, Write};


fn main() {
    println!("TicTacToe");
    let mut game = TicTacToe::new(3);
    while !game.has_game_ended {
        game.turn();
    }
    game.display_board();
    println!("Game completed!");
    if game.winner.is_some() {
        println!("The winner is player {:?}", game.winner.unwrap());
        return;
    }
    println!("The game is draw!");
}

struct TicTacToe {
    has_game_ended: bool,
    is_draw: bool,
    winner: Option<Player>,
    current_player: Player,
    board_size: usize,
    board: Vec<Vec<Option<Player>>>
}

struct Position {
    row: usize,
    column: usize
}

impl TicTacToe {
    fn new(board_size: usize) -> Self {
        TicTacToe { 
            has_game_ended: false,
            is_draw: false,
            winner: None, 
            current_player: Player::X, 
            board_size: board_size,
            board: (0..board_size).map(|_| (0..board_size).map(|_| None).collect()).collect()
        }
    }

    fn turn(&mut self) {
        self.display_board();
        let maybe_position = self.get_position();
        if !self.is_move_valid(&maybe_position) {
            println!("Invalid position! expected valid row and column position (any number from 1 to {}) and is empty", self.board_size);
            return;
        }
        let position = maybe_position.unwrap();
        self.execute_move(position);
        self.switch_player();
        self.evaluate_board();
    }

    fn get_position(&self) -> Option<Position> {
        print!("({:?}) Input row: ", self.current_player);
        io::stdout().flush().unwrap();
        let maybe_row = self.get_user_input();
        if maybe_row.is_none() {
            return None;
        }
        print!("({:?}) Input column: ", self.current_player);
        io::stdout().flush().unwrap();
        let maybe_column = self.get_user_input();
        if maybe_column.is_none() {
            return None;
        }
        Some(
            Position { 
                row: maybe_row.unwrap() - 1, 
                column: maybe_column.unwrap() - 1 
            }
        )
    }

    fn get_user_input(&self) -> Option<usize> {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read keyboard input");
        let maybe_position = user_input.trim().parse::<usize>();
        if maybe_position.is_err() {
            return None;
        }
        return Some(maybe_position.unwrap());
    }

    fn is_move_valid(&self, position: &Option<Position>) -> bool {
        if position.is_none() {
            return false;
        }
        let real_pos = position.as_ref().unwrap();
        if real_pos.column >= self.board_size 
        || real_pos.row >= self.board_size {
            return false;
        }
        self.board[real_pos.row][real_pos.column].is_none()
    }

    fn execute_move(&mut self, position: Position) {
        self.board[position.row][position.column] = Some(self.current_player)
    }

    fn switch_player(&mut self) {
        if self.current_player == Player::X {
            self.current_player = Player::O
        } else {
            self.current_player = Player::X
        }
    }

    fn evaluate_board(&mut self) {
        // check if board is filled with same player horizontally
        for row in &self.board {
            let clean_row: Vec<Player> = row.iter()
                .filter(|col| !col.is_none())
                .map(|col| col.unwrap())
                .collect();
            if clean_row.len() < self.board_size {
                continue;
            }
            let first_column = clean_row[0];
            if clean_row.iter().all(|col| *col == first_column) {
                self.winner = Some(first_column);
                self.has_game_ended = true;
                return;
            }
        }
        // check if board is filled with same player vertically
        for vertical_index in 0..self.board_size - 1 {
            let clean_column: Vec<Player> = (0..self.board_size - 1)
                .map(|horizontal_index| self.board[vertical_index][horizontal_index])
                .filter(|v| !v.is_none())
                .map(|v| v.unwrap())
                .collect();
            if clean_column.len() < self.board_size {
                continue;
            }
            let first_value = clean_column[0];
            if clean_column.iter().all(|v| *v == first_value) {
                self.winner = Some(first_value);
                self.has_game_ended = true;
                return;
            }
        }
        
        // check if board is filled with same player diagonally

        // check if board is full (to check draw condition)
        if self.board.iter().all(|row| row.iter().all(|col| !col.is_none()))  {
            self.has_game_ended = true;
            self.is_draw = true;
            return;
        }
    }

    fn display_board(&self) {
        for row in &self.board {
            let stringified: Vec<&str> = row.iter().map(|col| {
                if col.is_none() {
                    return "";
                }
                if col.unwrap() == Player::X {
                    return "X";
                }
                return "O";
            }).collect();
            println!("{:?}", stringified);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O
}

impl AsRef<str> for Player {
    fn as_ref(&self) -> &str {
        match self {
            Player::X => "X",
            Player::O => "O"
        }
    }
}