use csv::Reader;
use std::error::Error;

use crate::film::Film;

pub fn get_watchlist(path_to_watchlist_csv: &String) -> Result<Vec<Film>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path_to_watchlist_csv)?;
    let films = rdr
        .records()
        .into_iter()
        .map(|result| result.unwrap())
        .map(|record| Film {
            title: record[1].to_string(),
            year: record[2].to_string(),
        })
        .collect::<Vec<Film>>();
    return Ok(films);
}
