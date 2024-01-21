use std::error::Error;
use csv::Reader;

use crate::film::Film;

pub fn get_watchlist(path_to_watchlist_csv: &String) -> Result<Vec<Film>, Box<dyn Error>> {
  let mut rdr = Reader::from_path(path_to_watchlist_csv)?;
  let mut films: Vec<Film> = Vec::new();
  for result in rdr.records() {
      let record = result?;
      let film = Film {
        title: record[1].to_string(),
        year: record[2].to_string(),
      };
      films.push(film);
  }
  return Ok(films);
}