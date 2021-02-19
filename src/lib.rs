use std::{env, error::Error, fs::File};


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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for record in rdr.records() {
        let parsed = record?;
        println!("{:?}", parsed);
    }

    Ok(())
}