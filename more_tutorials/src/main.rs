/*** 

more Rust tutorials 
commenting out sections as i go


***/


fn main() {
	// sec 1
	// not variables, but variable bindings and "patterns"
    //let x = 5;
    //let (x,y) = (1,2);
    //let x: i32 = 5; //"x is a binding with the type i32 and the value five. (eyeroll, Computer Science)"

    //sec 2
    // this will throw an error, must give a value before you can use
    // (also, I like this)
    //let x: i32;
    //println!("the value of x is {}",x); 
   
    //sec 3
    // shadowing and mutable bindings aren't the same
    //let mut x: i32 = 1; 	//x is 1
    //x = 7;					//x is 7
    //let x = x;				//we just shadowed x to x changing it to immutable and with the value 7
    //let y = 4;
    //let y = "I can also be bound to text"; //y is now of a different type, and still immutable
   
    print_number(5);

	// function pointers, is interesting, and might be useful, but also
	// a bit tricky maybe?
	let f = plus_one;
	let six = f(5);

	//arrays are also immutable by default
	let a = [1,2,3]; //a: [i32;3]
	let mut m = [1,2,3]; //m: pi32;3]
	//shorthand for init to all the same value
	let a2 = [0;20];
	println!("a has {} elements",a.len());
	let names=["Jessica","Brendan","Bridget","John"];
	println!("the second name is: {}", names[1]); // 0-based arrays.
	//AND BONUS, ARRAYS ARE BOUNDS-CHECKED at RUN-TIME!

	// slices seem interesting, for reading a line into memory or something
	let a3 = [0, 1, 2, 3, 4];
	let complete = &a3[..]; // A slice containing all of the elements in a
	let middle = &a3[1..4]; // A slice of a: only the elements 1, 2, and 3

	let x: (i32, &str) = (1, "hello"); //&str is a string slice

	let (x, y, z) = (1, 2, 3); //let "destructures" or "breaks up" the tuple, and assigns the bits to three bindings.


	let tuple = (1, 2, 3);

	let x = tuple.0;
	let y = tuple.1;
	let z = tuple.2;

	println!("x is {}", x);

	// if - else if - else blocks pretty normal () are not used, it seems. can do cooler things
	// in a single line than c/c++

	let x = 5;

	let y = if x == 5 { 10 } else { 15 }; // y: i32

	// loop { 
	//} //is an infinite loop 

	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool

	while !done {
	    x += x - 3;

	    println!("{}", x);

	    if x % 5 == 0 {
	        done = true;
	    }
	}

	for x in 0..10 {
    	println!("{}", x); // x: i32
	}
	// and other clever loop tricks



}

fn print_number(x: i32) {
	println!("x is {}", x);
}
// need to declare the types of function arguments. 
fn print_sum(x: i32, y: i32){
	println!("the sum is: {}", x+y);
}
// when you return a value, need to define type as well
// functions return exactly one value. 
fn add_one(x: i32) -> i32 {
	x + 1 //no semicolon here because this is an expression which returns a value, unlinke a statement.
}
// this will take me some time because it's not clear once we get more
// complicated functions how we would return a value, unless we pass
// mutable references
// poor style but works
fn bad(x: i32) -> i32 {
	return x + 1;
}

// https://doc.rust-lang.org/book/references-and-borrowing.html
// ownership, and scope around variables i think i get.
// but we'll see once i get going

// and it looks like i was right about passing references
// except that the values cannot be changed at all.
// unless they are explicitly mutable
// going to take using for a little bit to understand the details



































