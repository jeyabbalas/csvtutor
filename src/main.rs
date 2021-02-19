use std::io;

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for record in rdr.records() {
        let parsed = record.unwrap();
        println!("{:?}", parsed);
    }
}
