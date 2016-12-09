// State Game!
//
// Description: Making it for the kiddos, recreating a game my grandfather made in DOS
//  Will pick a state, ask user to input Capital, Abbreviation, or Largest City. Will
//  write with several options, including if we want to do: Capitals, Abbreviations,  
//  Largest City or all three!
//
// Remarks: Each section will keep track of the number of right answers, the number of guesses
//  and after 5 wrong guesses (or more), give the user the choice to skip and be given the right
//  answer. Also keep track of number of skips.

extern crate csv;
extern crate rustc_serialize;
extern crate rand;

use std::io;
use std::path::Path;
use rand::{thread_rng, Rng};

#[derive(RustcDecodable, Debug)]
	struct AnswerRecord {
	id: u32,
    state: String,
    abbrev: String,
    capital: String,
    lgst_city: String,
}

fn main() {
	// PART 1 INITIAL INPUT
	// Ask user to input which option they want to play: 
	// 1 -> Capitals only
	// 2 -> State Abbreviations only
	// 3 -> Largest Cities only
	// 4 -> All three!
    println!("Welcome to the State Game!");
    println!("Before we begin, please tell me your name.");
    let mut player = String::new();
    io::stdin().read_line(&mut player).expect("Failed to read name");
    println!("Hello {}!", player.trim());
    println!("Please choose which option you want to play. Enter 1, 2, 3, or 4:");
    println!("1: State Capitals only\n2: State Abrreviations only\n3: Largest Cities only\n4: ALL THREE!");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read option");
    let option: u32 = match option.trim().parse() {
    	Ok(num) => num,
    	Err(_) => panic!("Failed to convert option")
    };
    match option {
    	1 => println!("State Capitals it is!"),
    	2 => println!("State Abrreviations here we go!"),
    	3 => println!("Largest Cities it is!"),
    	4 => println!("Feeling confident? Let's do them all!"),
    	_ => panic!("Unknown option"),
    };

    // PART 2 READ ANSWER CSV
	let path = Path::new("./input-data/answers.csv");
	let mut state_answer_records = vec![];
    // set up the reader
    let mut rdr = csv::Reader::from_file(&path).unwrap();
    for state_rec in rdr.decode() {
        let state_rec: AnswerRecord = state_rec.unwrap();
        //println!("{:?}",state_rec );
        state_answer_records.push(state_rec);
	}

	// PART 3 START TO PLAY

	// 1. State Capitals only
	if option == 1 { 
		do_state_capitals(&state_answer_records, &player);
	}
	else {
		println!("Not ready yet, try again soon.")
	}

	// 2. State Abbrev. only

	// 3. Largest Cities only

	// 4. All three.


}

// All the shared functions are here
fn is_player_right(input_string: &str, answer_string: &str) -> bool {
	// redo as a match statement?
    if input_string == answer_string {
    	return true;
    }
    else {
    	return false;
    }
}


// All the main do_ methods are below here

// Description: Randomly pick an id and ask for user to input the Capital. Check the answer,
//  count correct answers, and number of guesses to get to the answer.
//  after 5 incorrect answers, ask if user wants to skip (and be told the answer)
fn do_state_capitals(state_answer_records: &Vec<AnswerRecord>, player: &String) {
	// perhaps put in a record type for tracking? since they will be the 
	// same across functions
	let mut num_correct = 0; 		// total number of correct answers
	let mut num_guess_tot = 0;		// total number of guesses
	let mut num_skip = 0;			// total number of skips
	// keep this outside of whatever record type we decide on
	let mut num_guess_this = 0;		// total number of guesses this question
	
	// generate a random ordering of the numbers 0-50 to use as our indices for the state_answer_records
	let mut rng = thread_rng();
	let mut rnd_states = (0..50).collect::<Vec<usize>>(); // collect the numbers into a vector
	rng.shuffle(&mut rnd_states);	// and randomly shuffle them
    

    for s in 0..50 {
    	let state_id = rnd_states[s];
		println!("What is the capital of {}?", state_answer_records[state_id].state);		

		loop { // loop for guessing the answer of one
			let mut guess = String::new();
		    io::stdin().read_line(&mut guess).expect("Failed to read guess");
		    //println!("You guessed {}!", guess.trim());
		    //println!("The right answer is {}", state_answer_records[state_id].capital);
		    
		    //let check = if guess.trim() == state_answer_records[state_id].capital{true} else {false};
		    let check = is_player_right(guess.trim(), state_answer_records[state_id].capital.as_ref());

		    if check == true {
		    	println!("YOU'RE RIGHT!");
		    	num_guess_this = 0;
		    	num_correct += 1;
		    	num_guess_tot += 1;
		    	break;
		    }
		    else {
		    	println!("Sorry, guess again! (Remember, spelling and capitalization count)");
		    	num_guess_tot += 1;
		    	num_guess_this += 1;
		    }

		    if num_guess_this > 5 {
		    	println!("Do you want to skip this state? [Y or N]");
		    	let mut skip = String::new();
		    	io::stdin().read_line(&mut skip).expect("Failed to read skip");
		    	if skip.trim() == "Y" {
		    		num_guess_this = 0;
		    		num_skip += 1;
		    		break;
		    	}
		    	else {
		    		println!("Guess again, {}", player.trim());
		    	}
		    	
		    }
		}//end guess one answer loop.


	}// end loop over states

	println!("Final Results for {}", player.trim());
	println!("You got {} right and made {} bad guesses and skipped {} states",num_correct, num_guess_tot-num_correct, num_skip );
	if num_correct == 50 && num_guess_tot == 50 && num_skip == 0 {
		println!("!!!! HOORAY !!!!!");
		println!("A PERFECT GAME");
		println!("THREE CHEERS FOR {}",player.trim() );
	}

}
















