use std::io::{self, Write};


fn main() {
    println!("TicTacToe");
    let mut game = TicTacToe::new(3);
    while !game.has_game_ended {
        game.turn();
    }
    game.display_board();
    println!("Game completed!");
    // Taming the unwrap() with Pattern Matching
    if let Some(winning_player) = game.winner {
        println!("The winner is player {:?}", winning_player);
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
    board: Vec<Vec<Option<Player>>>,
    turn_count: usize
}

// When you add #[derive(Copy, Clone)] to Position, you are telling the compiler:
// "This data is so small and simple that if I pass it to a function, just instantly duplicate the bits on the stack."
//
// When you call self.is_move_valid(maybe_position),
// the CPU just shuffles a few bytes from one stack frame to another (or keeps them entirely inside the CPU's ultra-fast registers).
// You might think that passing a reference (&Option<Position>) is faster because you aren't duplicating data.
// However, for tiny data structures like this, copying by value is actually faster than passing a reference.
// The Golden Rule for Structs:
// If your struct only contains simple stack data (integers, booleans, floats, or simple enums),
// always derive(Copy) and pass it by value.
// Only use references (&) for complex structs that contain heap-allocated data like String or Vec,
// where copying would be a massive performance hit.
#[derive(Copy, Clone)]
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
            board_size,
            board: vec![vec![None; board_size]; board_size],
            turn_count: 0
        }
    }

    fn turn(&mut self) {
        self.display_board();
        let maybe_position = self.get_position();
        if !self.is_move_valid(maybe_position) {
            println!("Invalid position! expected valid row and column position (any number from 1 to {}) and is empty", self.board_size);
            return;
        }
        let position = maybe_position.unwrap();
        self.execute_move(position);
        self.increment_turn_count();
        self.evaluate_board();
        self.switch_player();
    }

    fn increment_turn_count(&mut self) {
        self.turn_count+=1;
    }

    fn get_position(&self) -> Option<Position> {
        print!("({:?}) Input row: ", self.current_player);
        io::stdout().flush().unwrap();
        // The `?` says: "If this is Some, give me the value. If it's None, instantly return None from this function."
        let maybe_row = self.get_user_input()?;
        if maybe_row == 0 {
            return None;
        }
        print!("({:?}) Input column: ", self.current_player);
        io::stdout().flush().unwrap();
        let maybe_column = self.get_user_input()?;
        if maybe_column == 0 {
            return None;
        }
        Some(
            Position { 
                row: maybe_row - 1,
                column: maybe_column - 1
            }
        )
    }

    fn get_user_input(&self) -> Option<usize> {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read keyboard input");
        // instead of manually checking for error and returning None,
        // .ok() parses to a Result, converts to an Option, and returns it
        user_input.trim().parse().ok()
    }

    fn is_move_valid(&self, position: Option<Position>) -> bool {
        let Some(real_pos) = position else {
            return false;
        };
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
        // skip calculating if the turn count isn't yet possible to calculate win
        if self.turn_count < (self.board_size * 2) - 1 {
            return;
        }

        // check if board is filled with same player horizontally
        for row in &self.board {
            if row.iter().all(|col| *col == Some(self.current_player)) {
                self.set_winner();
                return;
            }
        }
        // check if board is filled with same player vertically
        for col_index in 0..self.board_size {
            if self.is_winning_row(|_| col_index) {
                self.set_winner();
                return;
            }
        }
        
        // check if board is filled with same player diagonally
        // major diagonal
        if self.is_winning_row(|idx| idx) {
            self.set_winner();
            return;
        }
        // minor diagonal
        let last_index = self.board_size - 1;
        if self.is_winning_row(|idx| last_index - idx) {
            self.set_winner();
            return;
        }

        if self.is_full()  {
            self.set_draw();
            return;
        }
    }

    // either horizonal, vertical, or diagonal row
    fn is_winning_row(&self, secondary_index_extractor: impl Fn(usize) -> usize) -> bool {
        (0..self.board_size).all(|idx| {
            let secondary_index = secondary_index_extractor(idx);
            self.board[idx][secondary_index] == Some(self.current_player)
        })
    }

    fn set_winner(&mut self) {
        self.winner = Some(self.current_player);
        self.has_game_ended = true;
    }

    fn set_draw(&mut self) {
        self.is_draw = true;
        self.has_game_ended = true;
    }

    // check if board is full (to check draw condition)
    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|col| col.is_some()))
    }

    fn display_board(&self) {
        println!();
        for row in &self.board {
            for col in row {
                let symbol = match col {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => '_'
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O
}