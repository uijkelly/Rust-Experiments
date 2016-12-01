// Jessica A Kelly
//
// Nov/Dec 2016
//
// https://projecteuler.net/archives
//
// Description: Practice using Project Euler problems
//
// Remarks: Take argument for what problem to sovle in the form of a flag -p 
//   and a number to indicate the problem so ./proj_euler -p 1 would be problem one.
//   then we should first print the problem description.
//   Example of how to use getopts crate: https://doc.rust-lang.org/getopts/getopts/

extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -p [NUMBER]", program);
    print!("{}", opts.usage(&brief));
}

// Description: Main entry point. Checks that required argument -p is passed.
//  Will end execution and print usage if it is not.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "", "REQUIRED give the project euler number to solve", "NUMBER");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") || matches.opt_count("p") <=0 {
        print_usage(&program, opts);
        return;
    }
    let opt = matches.opt_str("p");
    //println!("{:?}",opt);
    solve_problem(opt);
}

// Description: Get the value of the option and call proper function to solve the problem indicated. 
//  If problem has not been solved yet, then say so.
fn solve_problem(opt: Option<String>) {
	// get the value of the option passed
    let opt_val = match opt {
        Some(x) => x,
        None => panic!("should never get here"),
    };
    // decide which solution to run
    if opt_val == "1" {
    	problem_1();
    }
    else if opt_val == "2" {
    	problem_2();
    }
    else if opt_val == "3" {
    	problem_3();
    }
    else if opt_val == "4" {
    	problem_4();
    }
    else {
    	println!("not solved yet");
    }   
}

// Remarks: Best way to calculate the sum?
fn problem_1() {
	println!("PROBLEM 1: Find the sum of all the multiples of 3 or 5 below 1000.");
	let mut sum = 0;
	for num in 1..1000 {
		// match on number mod 3 and number mod 5 is 0.
		match (num % 3, num % 5) {
			  (0,0) => sum += num,
			  (0,_) => sum += num,
			  (_,0) => sum += num,
			     _  => sum=sum, // do nothing
		}
	}
	println!("SOLUTION: The sum is {}", sum);
}

// Remarks: To do sum over the array/vector
//  http://codeinreview.com/83/sum-or-product-of-an-array-in-rust/
//
//  However, try to do this better than generating a vector and then
//  or simultanously summing it. Try an iterator. http://rustbyexample.com/trait/iter.html
//  though it seems we still will need a vector to iterate over.
fn problem_2() {
	println!("PROBLEM 2: By considering the terms in the Fibonacci 
	sequence whose values do not exceed four million, find the sum of the even-valued terms.");

	// NOTE THAT THESE ARE LOCAL TO THIS FUNCTION
	#[derive(Debug)]
	struct Fibonacci {
	    curr: u32,
	    next: u32,
	}

	// Implement `Iterator` for `Fibonacci`.
    // The `Iterator` trait only requires a method to be defined for the `next` element.
	impl Iterator for Fibonacci {
	    type Item = u32;

	    // Here, we define the sequence using `.curr` and `.next`.
	    // The return type is `Option<T>`:
	    //     * When the `Iterator` is finished, `None` is returned.
	    //     * Otherwise, the next value is wrapped in `Some` and returned.
	    fn next(&mut self) -> Option<u32> {
	        let new_next = self.curr + self.next;

	        self.curr = self.next;
	        self.next = new_next;

	        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
	        // will never return `None`, and `Some` is always returned.
	        Some(self.curr)
	    }
	}
	// Returns a Fibonacci sequence generator
	fn fibonacci() -> Fibonacci {
	    Fibonacci { curr: 1, next: 1 }
	}

	// Now that the iterator is set up, we can create a fibonacci sequence, generate
	// more in the list until we hit the limit and then sum everything up!
    let mut fib = fibonacci();
    let limit = 4000000;
    
    let mut fib_vec = vec![]; //seed with the first one
    loop {
    	fib.next();
    	if fib.curr <= limit {
    		match fib.curr % 2{
    			0 => fib_vec.push(fib.curr),
    			_ => fib.curr = fib.curr,
    		}
    		
    	}
    	else {
    		break;
    	}
    }
    println!("fibonacci seq of even numbers is {:?}",fib_vec);
    // summing it up
    let sum = fib_vec.iter().fold(0, |acc, &x| acc + x);

	println!("SOLUTION: The sum is {}", sum)
}

// Remarks: Doing it first how I would think about it and solve.
fn problem_3() {
	println!("PROBLEM: What is the largest prime factor of the number 600851475143?");
	// find the upper limit of what number to look for;
	// this all needs to be a function that returns a number to be added to the
	// vector so that we can call it not just for the main number, but for each factor 
	// as well because we need to know if they are prime.
	// so two functions factor_number(int)->vec <int> and is_prime(int) -> bool
	// trying to combine into one function that calls itself could become a recursive
	// nightmare. so going for clear over short. 

	fn factor_number(num: i64) -> Vec<i64> {
		let upper_lim = (num as f64).sqrt() as i64 + 1;
		let mut factors: Vec<i64> = Vec::new();

		for i in 2..upper_lim{
			if num % i == 0 {
				factors.push(i);
			}
		}
		return factors;
	}

    let val: i64 = 600851475143;
	
	let val_factors = factor_number(val);
	println!("Factors from function are: {:?}", val_factors);

	let mut prime_factors: Vec<i64> = Vec::new();
	for val in &val_factors {
		let factor_of_factor = factor_number(*val);
		println!("Factors of {} are {:?}",val,factor_of_factor);
		if factor_of_factor.len() == 0{
			prime_factors.push(*val);
		}
	}
	
	println!("Prime factors are: {:?}", prime_factors);
	// no need to sort because we know the last element is going to be the largest
	// based on our looping!
	println!("SOLUTION: Largest prime factor is: {:?}", prime_factors.last());
}

fn problem_4(){
	println!("PROBLEM: ");

	println!("SOLUTION: ");
}
