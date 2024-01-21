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

    let jf_client = jellyfin::init();
    let jf_user = match jellyfin::login(&jf_client) {
        Ok(user) => user,
        Err(error) => {
          eprintln!("Failed to login to JellyFin: {}", error);
          return ExitCode::from(3);
        }
      };

    for film in &watchlist {
        let film_title = format!("{} ({})", film.title, film.year);
        let found = jellyfin::is_film_on_jellyfin(&jf_client, &film, &jf_user);
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
