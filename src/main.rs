extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number Is: {}", secret_num);

    loop {
	    println!("Please input your guess:");

	    let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	    	.expect("Error reading input.");

	    let guess: u32 = guess.trim().parse()
	        .expect("Please type a number!");

	    println!("You guessed, {}", guess);

	    match guess.cmp(&secret_num) {
	    	Ordering::Less => println!("Too Small!"),
	    	Ordering::Greater => println!("Too Big!"),
	    	Ordering::Equal => {
	    		println!("You Win!");
	    		break;
	    	}
	    }	
    }
}
