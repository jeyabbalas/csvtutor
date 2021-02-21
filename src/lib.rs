use std::{env, error::Error};
use serde::{Serialize, Deserialize};


pub struct Config {
    filename: String,
    output_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(f) => {f},
            None => {return Err("Input csv file not specified.");},
        };

        let output_file = match args.next() {
            Some(f) => {f},
            None => {return Err("Output tsv file not specified.");},
        };

        Ok(Self{
            filename,
            output_file,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UsPopRecord {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

fn read_population_data(config: &Config) -> Result<Vec<UsPopRecord>, Box<dyn Error>> {
    let mut pop_data = Vec::new();

    // Configure csv reader: https://docs.rs/csv/1.1.5/csv/struct.ReaderBuilder.html 
    let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .has_headers(true)
                    .from_path(&config.filename)?;
    
    for record in rdr.deserialize() {
        let parsed: UsPopRecord = record?;
        pop_data.push(parsed);
    }

    Ok(pop_data)
}

fn population_gt(pop_data: Vec<UsPopRecord>, gt: u64) -> Vec<UsPopRecord> {
    let mut subpop_data: Vec<UsPopRecord> = Vec::new();

    for item in pop_data {
        let pop = match item.population {
            Some(p) => {p},
            None => {continue;},
        };

        if pop >= gt {
            subpop_data.push(item);
        }
    }

    subpop_data
}

fn write_population_data(config: &Config, pop_data: Vec<UsPopRecord>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .has_headers(true)
                .from_path(&config.output_file)?;
    
    for item in pop_data {
        wtr.serialize(item)?;
    }

    wtr.flush()?;

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let pop_data = read_population_data(&config)?;
    let subpop = population_gt(pop_data, 50000);
    write_population_data(&config, subpop)?;

    Ok(())
}