use std::env;
use std::process::ExitCode;

mod letterboxd;
mod jellyfin;
mod film;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the path to the CSV");
        return ExitCode::from(1);
    }

    let csv_file_name = &args[1];
    let watchlist = match letterboxd::get_watchlist(csv_file_name) {
        Ok(watchlist) => watchlist,
        Err(error) => {
            eprintln!("Failed to read the CSV: {}", error);
            return ExitCode::from(2) 
        },
    };

    for film in &watchlist {
        println!("{} ({})", film.title, film.year);
    }
    println!("\nTotal films: {}", watchlist.len());
    
    jellyfin::is_film_on_jellyfin(&watchlist[0]);

    ExitCode::SUCCESS
}
