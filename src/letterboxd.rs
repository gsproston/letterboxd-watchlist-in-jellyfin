use std::error::Error;
use csv::Reader;

pub struct Film {
  title: String,
  year: String
}

impl Film {
  pub fn get_title(&self) -> &String {
    return &self.title;
  }

  pub fn get_year(&self) -> &String {
    return &self.year;
  }
}

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