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
//
// To Do: - Organized the best we can? 
//        - Add option to do all three by state at once. (Alaska - Abbreviation, Capital, Largest City)

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
	// BASED ON THE STATE FIND:
	// 1 -> Capitals only
	// 2 -> State Abbreviations only
	// 3 -> Largest Cities only
	// 4 -> All three!
	// BASED ON THE XXXX FIND THE STATE
	// 5 -> Given the Capital
	// 6 -> Given the State abbrev
	// 7 -> Given the largest city 
	// 8 -> Given the Capital and State Abbrev
    println!("Welcome to the State Game!");
    println!("Before we begin, please tell me your name.");
    let mut player = String::new();
    io::stdin().read_line(&mut player).expect("Failed to read name");
    println!("Hello {}!", player.trim());
    println!("Please choose which option you want to play. Enter 1-8:");
    println!("Either Guess Given the State\n\t1: State Capital\n\t2: State Abrreviation\n\t3: Largest Cities\n\t4: ALL THREE!\n Or Guess the State:\n\t5: State from the Capital\n\t6: State from the Abbreviation\n\t7: State from the Largest City\n\t8: State from the Capital and Abbreviation");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read option");
    let option: i32 = match option.trim().parse() {
    	Ok(num) => num,
    	Err(_) => panic!("Failed to convert option")
    };
    match option {
    	1 => println!("State Capitals it is!"),
    	2 => println!("State Abrreviations here we go!"),
    	3 => println!("Largest Cities it is!"),
    	4 => println!("Feeling confident? Let's do them all!"),
    	5 => println!("Guess the state by capital!"),
    	6 => println!("Guess the state by abbreviation!"),
    	7 => println!("Guess the state by largest city!"),
    	8 => println!("Guess the state by capital and abbreviation"),
    	_ => panic!("Unknown option"),
    };

    // PART 2 READ ANSWER CSV
	let path = Path::new("/Users/jkelly/projects/rust/state_capitals/input-data/answers.csv");
	let mut state_answer_records = vec![];
    // set up the reader
    let mut rdr = csv::Reader::from_file(&path).unwrap();
    for state_rec in rdr.decode() {
        let state_rec: AnswerRecord = state_rec.unwrap();
        //println!("{:?}",state_rec );
        state_answer_records.push(state_rec);
	}

	// PART 3 START TO PLAY
	if option == 1 || option == 2 || option == 3 { 
		do_state_by_option(&state_answer_records, &player, &option);
	}
	else if option == 4{
		println!("First, abbreviations");
		let mut option2 = 2;
		do_state_by_option(&state_answer_records, &player, &option2 );
		println!("Now capitals");
		option2 = 1;
		do_state_by_option(&state_answer_records, &player, &option2 );
		println!("And finally, largest cities");
		option2 = 3;				
		do_state_by_option(&state_answer_records, &player, &option2 );
	}
	else if option == 5 || option == 6 || option == 7  || option == 8 {
		do_guess_state_by_option(&state_answer_records, &player, &option)
	}
	else {
		panic!("Oh no! Should never get here!");
	}

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

// I think I can generalize this to take whatever we are doing, and run it -- just changing the part that
// looks specifically at the capital portion of the state_answer_records struct.
fn do_state_by_option(state_answer_records: &Vec<AnswerRecord>, player: &String, option: &i32) {

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

    	let description = match *option {
    		1 => "capital",
    		2 => "abbreviation",
    		3 => "largest city",
    		_=> panic!("can't do all here!"),
    	};
		
		println!("What is the {} of {}?", description, state_answer_records[state_id].state);		

		loop { // loop for guessing the answer of one
			let mut guess = String::new();
		    io::stdin().read_line(&mut guess).expect("Failed to read guess");
		    //println!("You guessed {}!", guess.trim());
		    //println!("The right answer is {}", state_answer_records[state_id].capital);	    
		    //let check = if guess.trim() == state_answer_records[state_id].capital{true} else {false};
		    let check_against = match *option {
		    	1 => state_answer_records[state_id].capital.as_ref(),
		    	2 => state_answer_records[state_id].abbrev.as_ref(),
		    	3 => state_answer_records[state_id].lgst_city.as_ref(),
		    	_ => panic!("can't do this"),
		    };
		    let check = is_player_right(guess.trim(), check_against);

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
		    	println!("You've guessed wrong {} times. Do you want to skip this state? [Y or N]", num_guess_this);
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
		let check = num_correct + num_skip;
		//println!("check = {}", check);
		// give the user the chance to keep playing or to quit after 10. Also update them on progress
		if check % 10 == 0 && num_correct != 50{
			println!("Progress Report for {}", player.trim());
			println!("So far, you have gotten {} right, made {} bad guesses, and skipped {} states",num_correct, num_guess_tot-num_correct, num_skip );
			println!("Do you want to keep playing? [Y or N]");
			let mut end_now = String::new();
			io::stdin().read_line(&mut end_now).expect("Failed to read end now.");
			if end_now.trim() == "N" {
				num_guess_this = 0;
				break;
			}

		}
		


	}// end loop over states

	println!("Final Results for {}", player.trim());
	println!("You got {} right, made {} bad guesses, and skipped {} states",num_correct, num_guess_tot-num_correct, num_skip );
	if num_correct == 50 && num_guess_tot == 50 && num_skip == 0 {
		println!("!!!! HOORAY !!!!!");
		println!("A PERFECT GAME");
		println!("THREE CHEERS FOR {}",player.trim() );
	}

}

// Description: Per Bridget's request, adding options to guess the state based on captial, abbreviation, largest city
//
fn do_guess_state_by_option(state_answer_records: &Vec<AnswerRecord>, player: &String, option: &i32) {
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

    	let description = match *option {
    		5 => "capital",
    		6 => "abbreviation",
    		7 => "largest city",
    		8 => "capital and abbreviation",
    		_=> panic!("can't do anything else here!"),
    	};
		// for some reason compiler couldn't figure out what type i wanted.
		let mut opt8: String = state_answer_records[state_id].capital.to_string();
		opt8 += ", ";
		opt8 += &state_answer_records[state_id].abbrev.to_string();
		let opt8: &str = &opt8;

		let get_state_data: &str = match *option {
		    	5 => state_answer_records[state_id].capital.as_ref(),
		    	6 => state_answer_records[state_id].abbrev.as_ref(),
		    	7 => state_answer_records[state_id].lgst_city.as_ref(),
		    	8 => opt8,
		    	_=> panic!("can't do this"),
		    };

		println!("What is the state with the {} of {}?", description, get_state_data);		

		loop { // loop for guessing the answer of one
			let mut guess = String::new();
		    io::stdin().read_line(&mut guess).expect("Failed to read guess");
		    let check_against = state_answer_records[state_id].state.as_ref();
		    let check = is_player_right(guess.trim(), check_against);

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
		    	println!("You've guessed wrong {} times. Do you want to skip this one? [Y or N]", num_guess_this);
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
		let check = num_correct + num_skip;
		//println!("check = {}", check);
		// give the user the chance to keep playing or to quit after 10. Also update them on progress
		if check % 10 == 0 && num_correct != 50{
			println!("Progress Report for {}", player.trim());
			println!("So far, you have gotten {} right, made {} bad guesses, and skipped {} questions",num_correct, num_guess_tot-num_correct, num_skip );
			println!("Do you want to keep playing? [Y or N]");
			let mut end_now = String::new();
			io::stdin().read_line(&mut end_now).expect("Failed to read end now.");
			if end_now.trim() == "N" {
				num_guess_this = 0;
				break;
			}

		}
		


	}// end loop over states

	println!("Final Results for {}", player.trim());
	println!("You got {} right, made {} bad guesses, and skipped {} states",num_correct, num_guess_tot-num_correct, num_skip );
	if num_correct == 50 && num_guess_tot == 50 && num_skip == 0 {
		println!("!!!! HOORAY !!!!!");
		println!("A PERFECT GAME");
		println!("THREE CHEERS FOR {}",player.trim() );
	}

}













