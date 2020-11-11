extern crate csv;

use csv::StringRecord;
use std::io;

const BEFORE_CONTEXT_COUNT: usize = 4;

fn main() {
    let mut v: Vec<StringRecord> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result.expect("a CSV record");

        v.push(record.clone());

        if v.len() < BEFORE_CONTEXT_COUNT {
            continue;
        }

        for i in &v {
            println!("  {:?}", i);
        }

        println!("> {:?}", record);
        println!("");

        if v.len() > BEFORE_CONTEXT_COUNT {
            v.remove(0);
        }
    }
}
