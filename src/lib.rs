use std::{env, error::Error};
use serde::Deserialize;


pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(f) => {f},
            None => {return Err("Input csv file not specified.");},
        };

        Ok(Self{
            filename,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UsPopRecord {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Configure csv reader: https://docs.rs/csv/1.1.5/csv/struct.ReaderBuilder.html 
    let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .has_headers(true)
                    .from_path(config.filename)?;


    {
        // scoped because of mutable borrow of reader
        let header = rdr.headers()?;
        println!("{:?}", header);
    }

    for record in rdr.deserialize() {
        let parsed: UsPopRecord = record?;
        println!("{:?}", parsed);
    }

    Ok(())
}