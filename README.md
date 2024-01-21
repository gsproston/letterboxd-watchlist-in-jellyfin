# letterboxd-watchlist-in-jellyfin
Wee program that sees if any films in your Letterboxd watchlist are on JellyFin.

## Setup
[Get Rust](https://www.rust-lang.org/learn/get-started).

Export your [Letterboxd watchlist](https://letterboxd.com/watchlist/) in CSV format.

Set JellyFin credentials:
1. Copy "credentials.example.rs" in "src/jellyfin/".
1. Rename the copied file to "credentials.rs".
1. Edit the contents of the copied file and save.

## Run
```bash
cargo run <path_to_csv>
```
