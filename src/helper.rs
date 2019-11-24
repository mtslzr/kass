pub mod weather {

  use serde::Deserialize;
  use std::env;

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

  pub fn get_weather(zip: String) -> Result<(), reqwest::Error> {
    let api_key = env::var("API_KEY");
    let url = format!(
      "https://api.openweathermap.org/data/2.5/weather?zip={},us&APPID={}",
      zip,
      api_key.unwrap()
    );

    let mut call = reqwest::get(&url)?;
    let mut json: OpenWeatherMap = call.json()?;

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
}
