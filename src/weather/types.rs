use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<WeatherInfo>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: u32,
    pub name: String,
    pub cod: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherInfo {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: Option<i32>, // Make these optional
    pub grnd_level: Option<i32>, // Make these optional
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
    pub gust: Option<f64>, // Make this optional since it's not always present
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sys {
    pub r#type: u32,
    pub id: u32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

impl WeatherResponse {
    /// Get the main weather condition
    pub fn main_condition(&self) -> Option<&str> {
        self.weather.first().map(|w| w.main.as_str())
    }

    /// Get weather description
    pub fn description(&self) -> Option<&str> {
        self.weather.first().map(|w| w.description.as_str())
    }
}
