use super::types::WeatherResponse;
use crate::location::Location;
use crate::http_client::HttpClient;
use std::collections::HashMap;

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct WeatherClient {
    location: Location,
    units: String,
    api_key: String,
    http_client: HttpClient,
}

impl WeatherClient {
    pub fn new(location: Location, units: String, api_key: String) -> Self {
        Self {
            location,
            units: units.clone(),
            api_key,
            http_client: HttpClient::new(),
        }
    }

    pub async fn get_current_weather(&self, units: String, debug: bool) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("lat".to_string(), self.location.lat.to_string());
        params.insert("lon".to_string(), self.location.lon.to_string());
        params.insert("units".to_string(), self.units.to_string());
        params.insert("appid".to_string(), self.api_key.clone());

        if debug {
            let safe_params = params.iter()
                .map(|(k, v)| {
                    if k == "appid" {
                        format!("{}={}", k, "{api_key}")
                    } else {
                        format!("{}={}", k, v)
                    }
                })
                .collect::<Vec<_>>()
                .join("&");
            println!("🌐 OpenWeatherMap Endpoint: {}?{}", WEATHER_API_BASE_URL, safe_params);
        }

        let response = self.http_client.get(WEATHER_API_BASE_URL, params).await?;

        if response.status != 200 {
            return Err(format!("API request failed with status: {}", response.status).into());
        }

        let weather: WeatherResponse = serde_json::from_str(&response.body)?;

        // Print weather information (same as before)
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

// Keep the same helper functions
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