use std::env;
use std::error::Error;
use csv::Reader;

fn read_csv(csv_file_name: &String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(csv_file_name)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let csv_file_name = &args[1];
    read_csv(csv_file_name);
}
