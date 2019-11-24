extern crate reqwest;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct OpenWeatherMap {
    weather: Vec<Weather>,
    main: Temperature,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Temperature {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
    description: String,
}

fn convert_kelvin(temp: f64) -> f64 {
    temp * (9.0 / 5.0) - 459.67
}

fn get_weather(zip: String) -> Result<(), reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?zip={},us&APPID=22c1743ea805e4140071bcb33c417e35", zip);
    let mut json: OpenWeatherMap = reqwest::get(&url)?.json()?;
    json.main.temp = convert_kelvin(json.main.temp);
    json.main.temp_min = convert_kelvin(json.main.temp_min);
    json.main.temp_max = convert_kelvin(json.main.temp_max);

    println!("Current weather for {}", json.name);
    println!("{} / {:.0}°F", json.weather[0].main, json.main.temp);
    println!(
        "High {:.0}°F / Low {:.0}°F",
        json.main.temp_max, json.main.temp_min
    );
    Ok(())
}

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
    get_weather(zip_code);
}
