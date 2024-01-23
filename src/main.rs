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

    let path_to_watchlist_csv = &args[1];
    let mut watchlist = match letterboxd::get_watchlist(&path_to_watchlist_csv) {
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
    let jf_films = match jellyfin::get_all_films(&jf_client, &jf_user) {
        Ok(films) => films,
        Err(error) => {
          eprintln!("Failed to get JellyFin films: {}", error);
          return ExitCode::from(4);
        }
    };

    let mut films_found: Vec<Film> = Vec::new();
    let mut films_not_found: Vec<Film> = Vec::new();

    while let Some(film) = watchlist.pop() {
        let found = jf_films.iter().any(|jf_film| 
            jf_film.title.eq_ignore_ascii_case(&film.title) &&
            jf_film.year.eq_ignore_ascii_case(&film.year)
        );
        if found {
            films_found.push(film);
        } else {
            films_not_found.push(film);
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
