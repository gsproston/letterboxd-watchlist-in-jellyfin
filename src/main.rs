use std::env;
use std::error::Error;
use std::process::ExitCode;
use csv::Reader;

fn read_csv(csv_file_name: &String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(csv_file_name)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the path to the CSV");
        return ExitCode::from(1);
    }

    let csv_file_name = &args[1];
    match read_csv(csv_file_name) {
        Ok(()) => (),
        Err(error) => {
            eprintln!("Failed to read the CSV: {}", error);
            return ExitCode::from(2) 
        },
    };

    ExitCode::SUCCESS
}
