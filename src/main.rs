extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let mut score = 0;
    
    loop {

        score = play_game(score);

        println!("Your Score: {}", score);

        if to_continue() {
            continue;
        } else {
            break;
        }
    }
    
}

fn to_continue() -> bool {
    let mut play_again = String::new();
    let mut result = false;
    loop {
        let mut play_again = String::new();

        println!("Would you like to play again? (y/n)");

        io::stdin().read_line(&mut play_again)
            .expect("Error reading input.");

        let play_again = play_again.trim();

        match play_again.as_ref() {
            "y" => {
                result = true;
                println!("Let's play another round!");
                break;
            },
            "n" => {
                result = false;
                println!("Good bye!");
                break;
            },
            _ => {
                println!("{}, is not a valid input!", play_again);
                continue;
            }
        }   
    }
    result
}

fn play_game(score: i32) -> i32 {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    let mut new_score = 0;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Error reading input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed, {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                new_score = score + 1;
                break;
            }
        }
    }
    new_score
}
