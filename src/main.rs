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
        let film_title = format!("{} ({})", film.title, film.year);
        let found = jellyfin::is_film_on_jellyfin(film);
        if found {
            println!("{} - FOUND", film_title);
        } else {
            println!("{} - NOT FOUND", film_title);
        }
        // TODO remove
        break;
    }
    
    ExitCode::SUCCESS
}
