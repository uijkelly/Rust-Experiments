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
    else if opt_val == "5" {
    	problem_5();
    }
    else if opt_val == "6" {
    	problem_6();
    }
    else if opt_val == "7" {
    	problem_7();
    }
    else if opt_val == "8" {
    	problem_8();
    }
    else if opt_val == "9" {
    	problem_9();
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


    // another way to get at evens
    // for i in (1..100).filter(|&x| x % 2 == 0) {
    	//println!("{}", i);
	//}

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

// Remarks: A palindromic number reads the same both ways. The largest palindrome 
//  made from the product of two 2-digit numbers is 9009 = 91 * 99.
//
//  Working in the 900s first, to see if we find a palindromic number. Perhaps a non-general
//  solution, but it will work. the looping over in a loop again, gets tricky.
fn problem_4() {
	println!("PROBLEM: Find the largest palindrome made from the product of two 3-digit numbers.");
	// strategy is to start with 999 * 999, then 999 * 998 etc until we find the FIRST 
	// number that is a palindrome. Will need a function that checks if a number is a palindrome

	// Description: convert number to string, get the characters, reverse them then collect back to string.
	//  check if orig and reversed are equivalent
	fn is_palindrome(num: i64) -> bool {
		let rev_str = num.to_string().chars().rev().collect::<String>();
		let is = if num.to_string() == rev_str { true } else { false };
		return is;
	}

	// checking that the function works as expected
	println!("is 919 a palindrome? answer is {:?}", is_palindrome(919) );
	println!("is 8843 a palindrome? answer is {:?}", is_palindrome(8843) );

	// note that the last number 1000 is not included in the range, but the 900 is.
	// create array of numbers twice (?)
	let x_vec = (900..1000).rev().collect::<Vec<i64>>();
	// println!("x is {:?}",x );
	// loop over x and multiply 
	let mut found = false;
	for y in (900..1000).rev() {
		if !found {
			for x in &x_vec {
				let product = x * y;
				//println!("x is {}, y is {}, product is {}",x,y,product);
				if is_palindrome(product) {
					println!("SOLUTION: x is {}, y is {}, and the product is : {}", x, y, product);
					found = true; // so we break out of outer loop as well.
					break;
				}
			}
		}
	}	
}

// Remarks: Trying blindly and hope we don't get into an infinite loop!
//  It is not the fastest, but it does run. 
fn problem_5() {
	println!("PROBLEM What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?");
	let mut num = 20; // start at 20, because it can't be any smaller than that!
	loop {
		let mut count = 0;
		for i in 1..21 {
			if num % i == 0 { count = count + 1; } 
		}
		if count == 20 {
			println!("SOLUTION: The number is {}", num);
			break;
		}
		num += 20; // incrementing by 20 instead of 1 because we know if must be divisible by 20. 

	}
}

// Remarks: Could we reduce the number of lines?
fn problem_6() {

	println!("PROBLEM: Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");
	// The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385 (sum_sq)
	// The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025 (sum_nat_sq)
	// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
	
	// use mapping, and then a collect and fold.
	let natural = (1..101).collect::<Vec<i32>>();
	let sum_nat = natural.iter().fold(0, |acc, &x| acc + x);
	let sum_nat_sq = sum_nat.pow(2);
	let natural_sq: Vec<i32> = natural.into_iter().map(|x| x.pow(2)).collect(); // transform the natural vector into another vector
	//println!("{:?}", natural_sq );
	let sum_sq = natural_sq.iter().fold(0, |acc, &x| acc + x);
	println!("SOLUTION: sum_nat_sq {} - sum_sq {} = {}", sum_nat_sq, sum_sq, sum_nat_sq - sum_sq);
}

// Remarks: Remember your Abstract Algebra?
//   but until i dig that up, use the factor_number function from above.
//   copied here because it's local to that function and i want each function to
//   be self-contained.
// Note: This is pretty fast so won't worry about anything else if i don't have to!
fn problem_7 () {
	println!("PROBLEM: What is the 10,001st prime number?");
	let mut primes = vec![];
	fn factor_number(num: i64) -> Vec<i64> {
		let upper_lim = (num as f64).sqrt() as i64 + 1;
		let mut factors: Vec<i64> = Vec::new();

		for i in 2..upper_lim {
			if num % i == 0 {
				factors.push(i);
			}
		}
		return factors;
	}

	let mut i = 2;
	loop {
		let factors = factor_number(i);
		if factors.len() == 0 { primes.push(i);}
		if primes.len() == 10001 {
			println!("SOLUTION: {:?}", primes.last());
			break;
		}
		i = i+1;
	}	
}

// the number is (below, manually made into vector):
// change this number to an array and slice it by 13, remove any slice that has a 0 in it (ideally, but this is not implemented).
// then do the products and find the slice that has the biggest one
fn problem_8() {
	println!("PROBLEM:Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?");
    // just plop that number in a vector.
    let n = vec![7,3,1,6,7,1,7,6,5,3,1,3,3,0,6,2,4,9,1,9,2,2,5,1,1,9,6,7,4,4,2,6,5,7,4,7,4,2,3,5,5,3,4,9,1,9,4,9,3,4,
				9,6,9,8,3,5,2,0,3,1,2,7,7,4,5,0,6,3,2,6,2,3,9,5,7,8,3,1,8,0,1,6,9,8,4,8,0,1,8,6,9,4,7,8,8,5,1,8,4,3,
			 	8,5,8,6,1,5,6,0,7,8,9,1,1,2,9,4,9,4,9,5,4,5,9,5,0,1,7,3,7,9,5,8,3,3,1,9,5,2,8,5,3,2,0,8,8,0,5,5,1,1,
			 	1,2,5,4,0,6,9,8,7,4,7,1,5,8,5,2,3,8,6,3,0,5,0,7,1,5,6,9,3,2,9,0,9,6,3,2,9,5,2,2,7,4,4,3,0,4,3,5,5,7,
			 	6,6,8,9,6,6,4,8,9,5,0,4,4,5,2,4,4,5,2,3,1,6,1,7,3,1,8,5,6,4,0,3,0,9,8,7,1,1,1,2,1,7,2,2,3,8,3,1,1,3,
			 	6,2,2,2,9,8,9,3,4,2,3,3,8,0,3,0,8,1,3,5,3,3,6,2,7,6,6,1,4,2,8,2,8,0,6,4,4,4,4,8,6,6,4,5,2,3,8,7,4,9,
				3,0,3,5,8,9,0,7,2,9,6,2,9,0,4,9,1,5,6,0,4,4,0,7,7,2,3,9,0,7,1,3,8,1,0,5,1,5,8,5,9,3,0,7,9,6,0,8,6,6,
				7,0,1,7,2,4,2,7,1,2,1,8,8,3,9,9,8,7,9,7,9,0,8,7,9,2,2,7,4,9,2,1,9,0,1,6,9,9,7,2,0,8,8,8,0,9,3,7,7,6,
				6,5,7,2,7,3,3,3,0,0,1,0,5,3,3,6,7,8,8,1,2,2,0,2,3,5,4,2,1,8,0,9,7,5,1,2,5,4,5,4,0,5,9,4,7,5,2,2,4,3,
				5,2,5,8,4,9,0,7,7,1,1,6,7,0,5,5,6,0,1,3,6,0,4,8,3,9,5,8,6,4,4,6,7,0,6,3,2,4,4,1,5,7,2,2,1,5,5,3,9,7,
				5,3,6,9,7,8,1,7,9,7,7,8,4,6,1,7,4,0,6,4,9,5,5,1,4,9,2,9,0,8,6,2,5,6,9,3,2,1,9,7,8,4,6,8,6,2,2,4,8,2,
				8,3,9,7,2,2,4,1,3,7,5,6,5,7,0,5,6,0,5,7,4,9,0,2,6,1,4,0,7,9,7,2,9,6,8,6,5,2,4,1,4,5,3,5,1,0,0,4,7,4,
				8,2,1,6,6,3,7,0,4,8,4,4,0,3,1,9,9,8,9,0,0,0,8,8,9,5,2,4,3,4,5,0,6,5,8,5,4,1,2,2,7,5,8,8,6,6,6,8,8,1,
				1,6,4,2,7,1,7,1,4,7,9,9,2,4,4,4,2,9,2,8,2,3,0,8,6,3,4,6,5,6,7,4,8,1,3,9,1,9,1,2,3,1,6,2,8,2,4,5,8,6,
				1,7,8,6,6,4,5,8,3,5,9,1,2,4,5,6,6,5,2,9,4,7,6,5,4,5,6,8,2,8,4,8,9,1,2,8,8,3,1,4,2,6,0,7,6,9,0,0,4,2,
				2,4,2,1,9,0,2,2,6,7,1,0,5,5,6,2,6,3,2,1,1,1,1,1,0,9,3,7,0,5,4,4,2,1,7,5,0,6,9,4,1,6,5,8,9,6,0,4,0,8,
				0,7,1,9,8,4,0,3,8,5,0,9,6,2,4,5,5,4,4,4,3,6,2,9,8,1,2,3,0,9,8,7,8,7,9,9,2,7,2,4,4,2,8,4,9,0,9,1,8,8,
				8,4,5,8,0,1,5,6,1,6,6,0,9,7,9,1,9,1,3,3,8,7,5,4,9,9,2,0,0,5,2,4,0,6,3,6,8,9,9,1,2,5,6,0,7,1,7,6,0,6,
				0,5,8,8,6,1,1,6,4,6,7,1,0,9,4,0,5,0,7,7,5,4,1,0,0,2,2,5,6,9,8,3,1,5,5,2,0,0,0,5,5,9,3,5,7,2,9,7,2,5,
				7,1,6,3,6,2,6,9,5,6,1,8,8,2,6,7,0,4,2,8,2,5,2,4,8,3,6,0,0,8,2,3,2,5,7,5,3,0,4,2,0,7,5,2,9,6,3,4,5,0];
	// need to make sure that we set up the product(s) to be typed so that they can hold large enough numbers!
	let mut largest_product: i64 = 0;
	// not the most efficient, would like to take out anything we know is going to be 0 but
	// i have a feeling it will be fast enough

	for i in 0..n.len() - 12 {
	//for i in 0..1 {
		let thirteen = &n[i..i + 13];
		//println!("i={}",i );
		//println!("{:?}", thirteen );
		let product: i64 = thirteen.iter().fold(1, |acc, &x| acc * x);
        if product > largest_product { largest_product = product; }
	}
	// testing the idea
	//let thirteen = &n[1..13];
	//println!("first thirteen = {:?}", thirteen);
	//let product = thirteen.iter().fold(1, |acc, &x| acc * x);
	//println!("product = {}", product);
	println!("SOLUTION: The largest product is {}", largest_product);
}

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
// maybe i should use tuples? or a struct? 
fn problem_9() {
	println!("PROBLEM:");
	// check if the tuple is a Pythagorean triple.
	// is_pythagorean(a,b,c) -> bool should take three numbers and return true if a Pythagorean triple.
	fn is_pythagorean(triple: (i64, i64, i64)) -> bool {
		let (a,b,c) = triple;
		let is = if a.pow(2) + b.pow(2) == c.pow(2) { true } else { false };
		return is;
	}
	// test that function works as expected.
	let mut test = (3,4,5);
	let mut is = is_pythagorean(test);
	println!("is {:?} Pythagorean? {}", test, is);
	test = (5,7,8);
	is = is_pythagorean(test);
	println!("is {:?} Pythagorean? {}", test, is);

    // generate tuples that add to 1000 where a < b < c . 
    


	println!("SOLUTION:");
}








