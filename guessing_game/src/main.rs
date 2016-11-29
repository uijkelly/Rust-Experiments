/**

A first tutorial for Rust
From "the book"
https://doc.rust-lang.org/book/guessing-game.html

(And hooray "Rust has a stong, static type system.")

**/
extern crate rand;

use std::io; //makes sense, standard io library. boom.
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the [POSITIVE] number [BETWEEN 1 and 101]");

    let secret_number = rand::thread_rng().gen_range(1,101);

    //for debugging only. don't want to spoil the game. 
    //println!("The secret number is: {}", secret_number);
  
    loop {
	    println!("Please input your guess.");

	    let mut guess = String::new(); // use mut to allow this to be mutable, default is immutable (oooh fancy word)

	    io::stdin().read_line(&mut guess) //break into two lines for readability
	      .expect("Failed to read line"); //&mut is a reference, so still very much like c
	      								  //wll get a warning that we aren't using the generated Result
	    // shadow guess because it starts as a String, but we want to convert it. 
	    // we re-use the name guess, (instead of guess_str like we would have to do in c/c++)
	    let guess: u32 = match guess.trim().parse() { //c-style strings with newline at the end
	      Ok(num) => num,    // Result returned from parse is an enum Ok is success, Err is failure
	      Err(_) => continue,
	    };

	    println!("You guessed: {}", guess);

	    //this bit is interesting... 
	    match guess.cmp(&secret_number) {
	    	Ordering::Less		=> println!("Too small!"),
	    	Ordering::Greater   => println!("Too big!"),
	    	Ordering::Equal     => {
	    		println!("You win!");
	    		break; //quit when we win
	    	}
	    }
	}
}
