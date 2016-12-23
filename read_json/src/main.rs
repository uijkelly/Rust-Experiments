// Jessica A Kelly
//
// Reading and parsing JSON (and also CSV)
// 
// General Notes:
//  http://zsiciarz.github.io/24daysofrust/book/vol1/day6.html
//  https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/
//  https://github.com/serde-rs/serde
//
//  one JSON will have file paths that I'll then open and read in CSV data
//  another JSON will have several input records that we'll read

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("/Users/jkelly/projects/rust/read_json/src/data_elements.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
	
	println!("------------");
    let json = Json::from_str(&data).unwrap();
    //println!("{}", json.find_path(&["Address", "Street"]).unwrap());
    println!("data_elements JSON");
 	println!("{:?}",json );
 	println!("------------");

 	file = File::open("/Users/jkelly/projects/rust/read_json/src/file_params.json").unwrap();
 	data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json_file_params = Json::from_str(&data).unwrap();
    println!("file_params stagefile");
    println!("{}", json_file_params.find_path(&["stagefile"]).unwrap());
  	println!("------------");

}
