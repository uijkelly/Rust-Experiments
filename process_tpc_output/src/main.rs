//
// Jessica A Kelly
//
// Program to process TPC extract in csv form
// to time, assess possibilities etc
//

#![allow(dead_code)]
#![allow(unused_variables)] 
mod full_extract;
//mod FullOutRecord;
use std::path::Path;
extern crate csv;
extern crate rustc_serialize;


#[derive(RustcDecodable, RustcEncodable)]
struct OutRecord {
    id: f32, 
//    sex: String,
//    outcome: u32,
}



// Description: Main entry point. Sets up paths, reads the input csv, does transformations,
//  sets up and pushes to output vector(s) and writes out results.
// 
// Remarks: Can I move paths to be command line arguments?
//  TODO: try just reading in the full line insteady of the input record. Compiling takes 10 min
fn main() {
	// Create a path to the desired file
    let path = Path::new("/Users/jkelly/projects/data/tabexp_2.csv");
    let outp = Path::new("/Users/jkelly/projects/process_tpc_output/src/output_data.csv");
    // set up the reader
    let mut rdr = csv::Reader::from_file(&path).unwrap();
    let mut wtr = csv::Writer::from_file(&outp).unwrap();
    // set up a vector of result records.
    //let mut results = vec![];
    for record in rdr.decode() {
        let record: full_extract::full_extract::TPCRecord = record.unwrap();
        //println!(" reading from file. record id is {}", record.rtrnid_puf);

        // do something based on type of record.sex and then create an output record
        //let outcome = calc_outcome(&record.sex);
        // put outcome into a output record struct. 
        let outrec = OutRecord {id: record.rtrnid_puf};
        // and push to a vector
        // was results.push(outrec)
        //results.push(&outrec);

        //write here, instead of a separate loop?
        wtr.encode(outrec).expect("CSV writer error");
    }
    // write results -- here will be id, sex and outcome -- to a file
    // iterate over the resuts vector we created in the loop above and then write the file.
    //let mut wtr = csv::Writer::from_file(&outp).unwrap();
    //for r in results.into_iter() {                      
    //    println!("id is: {} sex is: {} outcome is: {}", r.id, r.sex, r.outcome); // just to test that we are here ok and saved right.
        //let writerec = // don't need
    //    wtr.encode(r).expect("CSV writer error");
    //}
}

