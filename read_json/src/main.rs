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
extern crate csv;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use std::path::PathBuf;

// these are going to be in the order in which
// they occur in the file.
// input record
#[derive(RustcDecodable)]
struct InRecord {
    id: String,
    dob: String,
    first_name: String,
    sex: String,
    zipcode: String,
}

fn main() {
    // set 
    let root = Path::new("/Users/jkelly/projects/rust/read_json/src");
    let e = env::set_current_dir(&root);
    if e.is_ok() {

    	let mut file = File::open("data_elements.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
    	
    	println!("------------");
        let json = Json::from_str(&data).unwrap();
        //println!("{}", json.find_path(&["Address", "Street"]).unwrap());
        println!("data_elements JSON");
     	println!("{:?}",json );
     	println!("------------");

     	
        // moving what i had coded here to a function.
        open_and_read_csv();
    }
    else {
        panic!("Did not set path");
    }
}
// Description: TO DO!
fn open_and_read_csv() {
    let mut file = File::open("file_params.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json_file_params = Json::from_str(&data).unwrap();
    println!("file_params stagefile");
    println!("{}", json_file_params.find_path(&["stagefile"]).unwrap());
    println!("------------");
    // set up the reader
    let obj = json_file_params.as_object().unwrap();


// but do have to treat the files differently as they have different formats, but this idea will work.
// phew.
    for (key, value) in obj.iter() {
        let path2 = match *value {
            Json::String(ref v) => v,
            _ => panic!("expected string"),
        };
        let mut rdr2 = csv::Reader::from_file(&path2).unwrap();
    
    }



}


