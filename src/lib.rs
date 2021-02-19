use std::{io, error::Error};


pub fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for record in rdr.records() {
        let parsed = record?;
        println!("{:?}", parsed);
    }

    Ok(())
}