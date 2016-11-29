// the other way is slow, and i'm not sure if it is the number of rows, or the size of the struct
use std::path::Path;
extern crate csv;

fn main() {
	let path = Path::new("/Users/jkelly/projects/data/tabexp_2.csv");
	let mut rdr = csv::Reader::from_file(&path).unwrap();
	for row in rdr.records() {
	    let row = row.unwrap();
	    //println!("{:?}", row); // even with this it still goes really fast. must be the struct!
	}
}
