use std::io::{self, Write};


fn main() {
    println!("TicTacToe Game");
    let mut game = TicTacToe::new(3);
    game.play();
}

struct TicTacToe {
    game_state: GameState
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O
}

type PrintableBoard = String;

enum GameState {
    Playing{ playing_state_machine: PlayingStateMachine },
    Won(Player, PrintableBoard),
    Draw(PrintableBoard)
}

struct PlayingStateMachine {
    current_player: Player,
    board: Vec<Vec<Option<Player>>>,
    board_size: usize,
    turn_count: usize
}

impl PlayingStateMachine {
    fn new(board_size: usize) -> Self {
        PlayingStateMachine {
            current_player: Player::X,
            board: vec![vec![None; board_size]; board_size],
            board_size,
            turn_count: 0
        }
    }

    fn turn(&mut self) -> bool {
        let maybe_position = self.get_position();
        let Some (position) = maybe_position else {
            println!("Invalid position! Please enter a whole number bigger than 0");
            return false;
        };
        if !self.is_move_valid(position) {
            println!("Invalid position! expected valid row and column position (any number from 1 to {}) and is empty", self.board_size);
            return false;
        }
        self.execute_move(position);
        self.increment_turn_count();
        true
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

    fn is_move_valid(&self, position: Position) -> bool {
        if position.column >= self.board_size
            || position.row >= self.board_size {
            return false;
        }
        self.board[position.row][position.column].is_none()
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

    // either horizonal, vertical, or diagonal row
    fn is_winning_row(&self, secondary_index_extractor: impl Fn(usize) -> usize) -> bool {
        (0..self.board_size).all(|idx| {
            let secondary_index = secondary_index_extractor(idx);
            self.board[idx][secondary_index] == Some(self.current_player)
        })
    }

    fn get_winner(&self) -> Option<Player> {
        // skip calculating if the turn count isn't yet possible to calculate win
        if self.turn_count < (self.board_size * 2) - 1 {
            return None;
        }

        // check if board is filled with same player horizontally
        for row in &self.board {
            if row.iter().all(|col| *col == Some(self.current_player)) {
                return Some(self.current_player);
            }
        }
        // check if board is filled with same player vertically
        for col_index in 0..self.board_size {
            if self.is_winning_row(|_| col_index) {
                return Some(self.current_player);
            }
        }

        // check if board is filled with same player diagonally
        // major diagonal
        if self.is_winning_row(|idx| idx) {
            return Some(self.current_player);
        }
        // minor diagonal
        let last_index = self.board_size - 1;
        if self.is_winning_row(|idx| last_index - idx) {
            return Some(self.current_player);
        }

        None
    }

    // check if board is full (to check draw condition)
    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|col| col.is_some()))
    }

    fn display_board(&self) {
        let board_string = self.board_to_string();
        println!("{}", board_string);
    }

    fn board_to_string(&self) -> PrintableBoard {
        let mut board_string = String::new();
        board_string.push('\n');
        for row in &self.board {
            for col in row {
                let symbol = match col {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => '_'
                };
                board_string.push(symbol);
                board_string.push(' ');
            }
            board_string.push('\n');
        }
        board_string
    }
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
            game_state: GameState::Playing {
                playing_state_machine: PlayingStateMachine::new(board_size)
            }
        }
    }

    fn play(&mut self) {
        loop {
            let next_state: Option<GameState> = match &mut self.game_state {
                GameState::Playing { playing_state_machine } => {
                    playing_state_machine.display_board();
                    let is_turn_valid = playing_state_machine.turn();
                    if !is_turn_valid {
                        None
                    } else if let Some(winner) = playing_state_machine.get_winner() {
                        Some(GameState::Won(winner, playing_state_machine.board_to_string()))
                    } else if playing_state_machine.is_full() {
                        Some(GameState::Draw(playing_state_machine.board_to_string()))
                    } else {
                        playing_state_machine.switch_player();
                        None
                    }
                }
                GameState::Won(winner, board_string) => {
                    println!("{}", board_string);
                    println!("Player {:?} won!", winner);
                    return;
                }
                GameState::Draw(board_string) => {
                    println!("{}", board_string);
                    println!("Game is Draw!");
                    return;
                }
            };

            if let Some(new_state) = next_state {
                self.game_state = new_state;
            }
        }
    }
}