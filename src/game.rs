use colored::Colorize;
use std::io::{self, Write};
use std::process::exit;

pub struct Game {
    pub board: Vec<Vec<char>>,
    players: Vec<char>,
    num_to_connect: usize,
    rows: usize,
    cols: usize,
    winning_places: Vec<(usize, usize)>,
    piece_count: usize,
}

impl Game {
    pub fn start_game() {
        let mut game: Game;

        if Game::ask_for_game_type() {
            game = Game::generate_custom_game()
        } else {
            game = Game::new(vec!['X', 'O'], 7, 6, 4)
        }

        while game.winning_places.is_empty() {
            for (index, player) in game.players.clone().iter().enumerate() {
                game.print_pieces();
                game.run_turn(index, *player);
                if game.calculate_match(*player) {
                    let win_message = format!("Player {} ({}) Wins!!", index + 1, player);
                    let border = "~".repeat(win_message.len());
                    game.print_pieces();
                    println!("{border}");
                    println!("{win_message}");
                    println!("{border}");
                    exit(0)
                }
                if game.piece_count == 0 {
                    println!("~~~~~");
                    println!("Tie!!");
                    println!("~~~~~");
                    exit(0)
                }
            }
        }
    }

    fn new(players: Vec<char>, cols: usize, rows: usize, num_to_connect: usize) -> Game {
        Game {
            board: vec![vec![' '; cols]; rows],
            winning_places: Vec::new(),
            players,
            num_to_connect,
            rows,
            cols,
            piece_count: cols * rows,
        }
    }

    fn run_turn(&mut self, index: usize, player: char) {
        println!(
            "Player {} ({}), where would you like to put your piece?",
            index + 1,
            player
        );
        loop {
            let position = Game::get_integer(0, Some(self.cols as i32));

            if position < 0 || position > self.cols as i32 {
                println!("Invalid Choice");
                continue;
            }

            if self.board[0][position as usize - 1] != ' ' {
                println!("Column Full")
            } else {
                self.place_piece(position as usize - 1, player);
                self.piece_count -= 1;
                return;
            }
        }
    }

    fn generate_custom_game() -> Game {
        let mut player_count = 0;
        let mut players: Vec<char> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        let mut num_to_connect = 0;

        println!("How many rows do you want? [min 3]");
        while rows == 0 {
            rows = Game::get_integer(3, None)
        }

        println!("How many columns do you want? [min 3]");
        while cols == 0 {
            cols = Game::get_integer(3, None)
        }

        println!("How many players do you want? It is recommended for a smaller amount of players on a smaller board [min 2]");
        while player_count == 0 {
            player_count = Game::get_integer(2, None)
        }

        println!("How many pieces need to connect for someone to win? This must be less than both row and column amount [min 3]");
        while num_to_connect == 0 {
            let amount = Game::get_integer(3, None);
            if amount > cols || amount > rows {
                println!("Invalid Input")
            } else {
                num_to_connect = amount
            }
        }

        for player in 0..player_count {
            println!("What symbol or letter should player {} be?", player + 1);
            loop {
                print!("> ");
                io::stdout().flush().unwrap();

                let mut choice = String::new();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("TODO: panic message");

                if choice.trim().len() > 1 {
                    println!("Invalid Choice");
                    continue;
                }

                if let Some(choice_char) = choice.trim().chars().next() {
                    if players.contains(&choice_char) {
                        println!("Character Taken")
                    } else {
                        players.push(choice_char);
                        break;
                    }
                } else {
                    println!("Invalid Choice");
                }
            }
        }

        Game::new(
            players,
            cols as usize,
            rows as usize,
            num_to_connect as usize,
        )
    }

    fn get_integer(min: i32, max: Option<i32>) -> i32 {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("TODO: panic message");

        let choice_int = choice.trim().parse().unwrap_or(-1);

        return if choice_int < min && choice_int > max.unwrap_or(1000) {
            println!("Invalid Choice");
            -1
        } else {
            choice_int
        };
    }

    /// Asks user for the game type they want
    /// returns true if type is custom
    fn ask_for_game_type() -> bool {
        println!("Would you like to use the default preset, or to customise your game? [d,c,?]");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("TODO: panic message");

            if choice.trim().to_ascii_lowercase() == "d" {
                return false;
            } else if choice.trim().to_ascii_lowercase() == "c" {
                return true;
            } else if choice.trim() == "?" {
                println!("The default game uses two players (X and O), has 6 rows and 7 columns and requires a match of 4 to win.");
                println!("In a custom game, these can all be changed.")
            } else {
                println!("Invalid choice")
            }
        }
    }

    fn print_pieces(&self) {
        println!();
        for (row_index, row) in self.board.iter().enumerate() {
            for (column_index, item) in row.iter().enumerate() {
                if self.winning_places.contains(&(row_index, column_index)) {
                    print!("[{}]", item.to_string().red())
                } else {
                    print!("[{}]", item)
                }
            }
            println!()
        }
        for _ in 0..self.cols {
            print!("---")
        }
        println!();
        for row in 1..self.cols + 1 {
            print!(" {row} ")
        }
        println!("\n")
    }

    fn place_piece(&mut self, place: usize, player: char) {
        for row in (0..self.rows).rev() {
            if self.board[row][place] == ' ' {
                self.board[row][place] = player;
                return;
            }
        }
    }

    fn calculate_match(&mut self, player: char) -> bool {
        // Check for horizontal matches
        for row in 0..self.rows {
            for col in 0..self.cols - self.num_to_connect + 1 {
                if self.board[row][col..col + self.num_to_connect]
                    .iter()
                    .all(|&x| x == player)
                {
                    for piece in col..col + self.num_to_connect {
                        self.winning_places.push((row, piece))
                    }
                    return true;
                }
            }
        }

        // Check for vertical matches
        for row in 0..self.rows - self.num_to_connect + 1 {
            for col in 0..self.cols {
                if (0..self.num_to_connect).all(|i| self.board[row + i][col] == player) {
                    for piece in row..row + self.num_to_connect {
                        self.winning_places.push((piece, col))
                    }
                    return true;
                }
            }
        }

        // Check for diagonal matches (top-left to bottom-right)
        for row in 0..self.rows - self.num_to_connect + 1 {
            for col in 0..self.cols - self.num_to_connect + 1 {
                if (0..self.num_to_connect).all(|i| self.board[row + i][col + i] == player) {
                    for i in 0..self.num_to_connect {
                        self.winning_places.push((row + i, col + i))
                    }
                    return true;
                }
            }
        }

        // Check for diagonal matches (top-right to bottom-left)
        for row in 0..self.rows - self.num_to_connect + 1 {
            for col in self.num_to_connect - 1..self.cols {
                if (0..self.num_to_connect).all(|i| self.board[row + i][col - i] == player) {
                    for i in 0..self.num_to_connect {
                        self.winning_places.push((row + i, col - i))
                    }
                    return true;
                }
            }
        }

        // No matches found
        false
    }
}
