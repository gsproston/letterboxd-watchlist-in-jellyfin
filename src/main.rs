use std::io::Write;
use std::env;
use std::process::ExitCode;

use film::Film;

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

    let mut films_found: Vec<Film> = Vec::new();
    let mut films_not_found: Vec<Film> = Vec::new();

    for film in &watchlist {
        print!(".");
        let _ = std::io::stdout().flush();
        
        let found = jellyfin::is_film_on_jellyfin(&jf_client, &film, &jf_user);
        let film_copy = Film {
            title: film.title.clone(),
            year: film.year.clone(),
        };
        if found {
            films_found.push(film_copy);
        } else {
            films_not_found.push(film_copy);
        }
    }

    println!("\n{} films NOT FOUND:", films_not_found.len());
    for film in &films_not_found {
        println!("{} ({})", film.title, film.year);
    }
    println!("\n{} films FOUND:", films_found.len());
    for film in &films_found {
        println!("{} ({})", film.title, film.year);
    }
    
    ExitCode::SUCCESS
}
