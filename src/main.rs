use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_VALUE: u8 = 10;
const MIN_VALUE: u8 = 1;
const MAX_TRIES: u8 = 5;

enum GameState {
    StartMenu,
    Playing,
    GameOver,
}

struct Game {
    state: GameState,
    secret_number: u8,
    tries: u8,
    player_wins: bool,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            state: GameState::StartMenu,
            secret_number: 0,
            tries: 0,
            player_wins: false,
            game_over: false,
        }
    }

    fn init(&mut self) {
        self.state = GameState::StartMenu;
        self.secret_number = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);
        self.tries = 0;
        self.player_wins = false;
        self.game_over = false;
    }

    fn update(&mut self) {
        match self.state {
            GameState::StartMenu => {
                println!("Guess the number!");
                println!("Please press enter to start!");
                io::stdin().read_line(&mut String::new()).unwrap();
                self.init();
                self.state = GameState::Playing;
            }
            GameState::Playing => {
                println!("You have {} tries left!", MAX_TRIES - self.tries);
                let guess = get_player_guess(MIN_VALUE, MAX_VALUE);
                self.tries += 1;
                match guess.cmp(&self.secret_number) {
                    Ordering::Less => println!("Too small"),
                    Ordering::Equal => {
                        self.player_wins = true;
                        self.state = GameState::GameOver;
                    }
                    Ordering::Greater => println!("Too big!"),
                }

                if self.tries == MAX_TRIES {
                    self.state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                if self.player_wins {
                    println!("You win!");
                    println!("You guessed the number in {} tries!", self.tries);
                } else {
                    println!(
                        "Game over! You failed to guess the number in {} tries!",
                        MAX_TRIES
                    );
                    println!("The number was {}", self.secret_number);
                }
                if continue_playing() {
                    self.state = GameState::StartMenu;
                } else {
                    println!("Thanks for playing!");
                    self.game_over = true;
                }
            }
        }
    }
}

fn get_player_guess(start: u8, end: u8) -> u8 {
    loop {
        println!("Please enter a number between {start} and {end}!");

        let mut player_input = String::new();

        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line!");

        let player_input: u8 = match player_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value entered!");
                continue;
            }
        };
        if (start..=end).contains(&player_input) {
            return player_input;
        } else {
            continue;
        }
    }
}

fn continue_playing() -> bool {
    println!("Do you want to play again? (Y)es or (N)o");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        if let Some(user_input) = user_input.chars().next() {
            let user_input = user_input.to_ascii_lowercase();
            if ['n', 'y'].contains(&user_input) {
                if user_input == 'y' {
                    return true;
                } else {
                    return false;
                }
            } else {
                println!("Please enter either (Y)es or (N)o.");
                continue;
            }
        } else {
            println!("Please enter either (Y)es or (N)o.");
        }
    }
}

fn main() {
    let mut guessing_game = Game::new();
    'game: loop {
        guessing_game.update();
        if guessing_game.game_over == true {
            break 'game;
        }
    }
}
