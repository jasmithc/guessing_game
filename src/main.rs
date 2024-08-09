use rand::Rng;
use std::cmp::Ordering;
use std::io;

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
    println!("Do you want to play again? Y\\N");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let user_input: char = user_input.chars().next().unwrap();
        let user_input = user_input.to_ascii_lowercase();
        // println!("You entered: {again_check}");

        if ['n', 'y'].contains(&user_input) {
            if user_input == 'y' {
                return true;
            } else {
                return false;
            }
        } else {
            println!("Please enter either Y\\n for yes or no.");
            continue;
        }
    }
}

fn main() {
    const MAX_VALUE: u8 = 10;
    const MIN_VALUE: u8 = 1;

    'game: loop {
        println!("Guess the number!");

        let secret_number: u8 = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);

        'round: loop {
            let guess = get_player_guess(MIN_VALUE, MAX_VALUE);

            println!("Your guess : {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Equal => {
                    println!("You win!");
                    break 'round;
                }
                Ordering::Greater => println!("Too big!"),
            }
        }

        if continue_playing() {
            continue 'game;
        } else {
            break 'game;
        }
    }
}
