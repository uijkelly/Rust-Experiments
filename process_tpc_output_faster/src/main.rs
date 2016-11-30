
#![allow(dead_code)]
#![allow(unused_variables)] 
//mod full_extract;
use std::path::Path;
extern crate csv;
extern crate rustc_serialize;



fn main() {
  //read_full();
  read_ten_obs();
  //read_full_vec();
}


// the other way is slow, and i'm not sure if it is the number of rows, or the size of the struct
// print out every 10000th line there are 280,000 records so that should be 28 lines or so. 
fn read_full() {
	let path = Path::new("/Users/jkelly/projects/data/tabexp_2.csv");
	let mut rdr = csv::Reader::from_file(&path).unwrap();
	let mut i = 0;
	for row in rdr.records() {
	    let row = row.unwrap();
	    i += 1;
	    match i%10000 {
	    	0 => println!("processed {} records",i),
	    	_ => i=i, // default condition .need something here, point is to do nothing.
	    };
	}
}

#[derive(RustcDecodable, Debug)]
struct InRecord {
    row_vec: Vec<f32>,
}

// do a read of a small 10 observation sample
// Remarks: Initially, this is skipping the first record, even though there are no headers, 
//  and there is no has_headers option (it seems for files)
//  and because it wants the file to be rectangular there's no good way around this. So, I
//  added a header.
//  THIS TOOK AGES TO FIGURE OUT, AND NOT SURE IF IT IS THE BEST.
//  STILL NEEDS AND ENUM OR SOMETHING FOR THE INDEX, SO I KNOW WHAT COLUMN 0, COLUMN 45 ETC ARE.
fn read_ten_obs() {
	let path = Path::new("/Users/jkelly/projects/data/small_obs.csv");
	let mut rdr = csv::Reader::from_file(&path).unwrap();
    // what i have is a vector or array for a sinlge row of 727?! floating point elements.
    let rows = rdr.decode().collect::<csv::Result<Vec<InRecord>>>().unwrap();
    // the line below gets just the first column 
    //let col1 = rdr.decode().collect::<csv::Result<Vec<f32>>>().unrwap();
    // println! ("data for first col is {:?}", col1);
    println!("data is {:?}", rows[0]);
    println!("first data element {}", rows[0].row_vec[0]);
}

// reads the full file (and with a header added by hand)
// using the same method as read_ten_obs above.
// taking over 2 minutes so it's not a winner either for this much data
fn read_full_vec(){
	let path = Path::new("/Users/jkelly/projects/data/tabexp_2.csv");
	let mut rdr = csv::Reader::from_file(&path).unwrap();
    // what i have is a vector or array for a sinlge row of 727?! floating point elements.
    let rows = rdr.decode().collect::<csv::Result<Vec<InRecord>>>().unwrap();
    println!("data is {:?}", rows[0]);
    println!("first data element {}", rows[0].row_vec[0]);

}