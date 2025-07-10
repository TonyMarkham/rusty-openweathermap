use super::types::WeatherResponse;
use crate::location::Location;

// https://api.openweathermap.org/data/2.5/weather?lat=42.44209&lon=-82.1993&appid={api_key}

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct WeatherClient {
    location: Location,
    units: String,
    api_key: String,
}

impl WeatherClient {
    pub fn new(location: Location, units: String, api_key: String) -> Self {
        Self {
            location,
            units: units.clone(),
            api_key,
        }
    }

    pub async fn get_current_weather(&self, units: String, debug: bool) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request = client
            .get(WEATHER_API_BASE_URL)
            .query(&[
                ("lat", self.location.lat.to_string()),
                ("lon", self.location.lon.to_string()),
                ("units", self.units.to_string()),
                ("appid", self.api_key.clone()),
            ])
            .build()?;

        if debug {
            // Hide the API key when printing
            let url_string = request.url().to_string();
            let safe_url = url_string.replace(&self.api_key, "{api_key}");
            println!("🌐 OpenWeatherMap Endpoint: {}", safe_url);
        }

        let response = client.execute(request).await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let weather: WeatherResponse = response.json().await?;

        println!("🌤️ Weather in {}", weather.name);
        println!("📍 Coordinates: ({}, {})", weather.coord.lat, weather.coord.lon);

        let temp_display = get_temperature_display(weather.main.temp, &units);
        println!("🌡️ Temperature: {}", temp_display);

        let wind_display = get_speed_display(weather.wind.speed, &units);
        println!("💨 Wind: {} at {}°", wind_display, weather.wind.deg);

        println!("☁️ Clouds: {}%", weather.clouds.all);

        if let Some(weather_info) = weather.weather.first() {
            println!("🌈 Conditions: {} ({})", weather_info.main, weather_info.description);
        }

        Ok(weather)
    }
}

fn get_temperature_display(temp: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1}°C", temp),
        "imperial" => format!("{:.1}°F", temp),
        "standard" | _ => format!("{:.1}°K", temp),
    }
}

fn get_speed_display(speed: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1} m/s", speed),
        "imperial" => format!("{:.1} mph", speed),
        "standard" | _ => format!("{:.1} m/s", speed),
    }
}
