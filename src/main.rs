extern crate reqwest;
mod helper;
pub use crate::helper::weather;
use std::io;

fn main() {
  println!("Welcome to Kass!");

  println!("Please enter a five-digit zip code (US only):");
  let mut zip_code = String::new();

  io::stdin()
    .read_line(&mut zip_code)
    .expect("Failed to parse input.");

  let _url = [
    "https://api.openweathermap.org/data/2.5/weather?q=",
    &zip_code,
    "&APPID=22c1743ea805e4140071bcb33c417e35",
  ]
  .concat();
  weather::get_weather(zip_code);
}
