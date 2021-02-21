# csvtutor

This repository follows [this tutorial on reading/writing CSV files using Rust](https://docs.rs/csv/1.1.5/csv/tutorial/index.html).

The CLI tool takes in US population data where each row contains city, state, population, latitude, and longitude. The tool filters cities with population greater than 50,000 and then writes it to an output file in tab-delimited format.

both reading and writing is done using serde.

## Usage

```
> ./target/debug/csvtutor < uspop.csv uspop_out.csv
```
