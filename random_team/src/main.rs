// To randomize standup order


extern crate rand;
use rand::{thread_rng, Rng};

fn print_todays_order(present: Vec<&str>) {
    println!("Today's standup order is: ");

    let full_team = ["Jessica", "Sybil", "Jeff", "Alyssa", "Kyle", "Paul", "Silke"];

    //let num_pres = present.len();
    let num_full = full_team.len();
    let mut rng = thread_rng();

    // collect the numbers into a vector of the full team size.
    let mut rnd_team = (0..num_full).collect::<Vec<usize>>(); 

    rng.shuffle(&mut rnd_team);	// and randomly shuffle them
    // now match to team members
    // but first need to check if in the present array before printing.
    for member in &rnd_team {
        if present.contains(&full_team[*member]) {
            println!("{}", full_team[*member])
        }   
    }
    // what if we send someone we don't have in the full team list?
    // loop over present and make sure we can find them in full team.
    for here in &present {
        if !full_team.contains(&here) {
            println!("unknown team member {:?}", here)
        }
    }

}

fn main() {
    // see: https://stackoverflow.com/questions/37285508/how-do-i-pass-a-vector-and-return-a-vector-of-strings
    // for notes about slices, strings, vectors and the like. 
    let present = ["Jessica", "Alyssa", "Sybil", "Chuck"];
    print_todays_order(present.to_vec());
}
