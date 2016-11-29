//
// Jessica A Kelly
//
// Attempting to do a simple file read of a CSV file:
// Contents of the file are totally made up and below:
//
// ID,DOB,FIRSTNAME,SEX,ZIPCODE
// 98-4332,19980101,Alice,F,02453
// 96-2185,19960101,Ben,M,21045
// 93-0723,19930101,Ricardo,M,21227
// 95-3425,19950101,Donald,M,22192
//
// General Notes: There is a handy CSV crate! 
//    https://github.com/BurntSushi/rust-csv
//    http://burntsushi.net/rustdoc/csv/struct.Writer.html
//    http://zsiciarz.github.io/24daysofrust/book/day3.html

use std::path::Path;
extern crate csv;
extern crate rustc_serialize;

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

#[derive(RustcDecodable, RustcEncodable)]
struct OutRecord {
    id: String, 
    sex: String,
    outcome: u32,
}

///
/// Description: Main entry point. This will read sample_data and save into an 
///  InRecord struct record (just a single record), do some simple data transformation
///  and save to an OutRecord struct and push to a vector of OutRecords. This vector
///  is then output to the output csv file
///
fn main() {
    // Create a path to the desired file
    let path = Path::new("/Users/jkelly/projects/read_csv/src/sample_data.csv");
    let outp = Path::new("/Users/jkelly/projects/read_csv/src/output_data.csv");

    // set up the reader
    let mut rdr = csv::Reader::from_file(&path).unwrap();
    // set up a vector of result records.
    let mut results = vec![];
    for record in rdr.decode() {
        let record: InRecord = record.unwrap();
        println!(" reading from file. id is: {} dob is: {} first_name is: {} sex is: {} zipcode is: {}",
            record.id, record.dob, record.first_name, record.sex, record.zipcode);

        // do something based on type of record.sex and then create an output record
        let outcome = calc_outcome(&record.sex);
        // put outcome into a output record struct. 
        let outrec = OutRecord {id: record.id, sex: record.sex, outcome: outcome};
        // and push to a vector
        results.push(outrec);
    }
    // write results -- here will be id, sex and outcome -- to a file
    // iterate over the resuts vector we created in the loop above and then write the file.
    let mut wtr = csv::Writer::from_file(&outp).unwrap();
    for r in results.into_iter() {                      
        println!("id is: {} sex is: {} outcome is: {}", r.id, r.sex, r.outcome); // just to test that we are here ok and saved right.
        //let writerec = // don't need
        wtr.encode(r).expect("CSV writer error");
    }
}

/// Description: Takes argument string and returns and unsigned integer. 
///  If sex is "F" then returns 1. Otherwise, returns 2
///
/// Remarks: Passing reference so that we can use the value here and then later.
/// 
fn calc_outcome(sex: &String) -> u32 {
  if sex == "F" {1} else {2}    
}
