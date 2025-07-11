use super::types::WeatherResponse;
use crate::location::Location;
use std::collections::HashMap;

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct WeatherClient {
    location: Location,
    units: String,
    api_key: String,
    #[cfg(not(target_arch = "wasm32"))]
    client: reqwest::Client,
}

impl WeatherClient {
    pub fn new(location: Location, units: String, api_key: String) -> Self {
        Self {
            location,
            units: units.clone(),
            api_key,
            #[cfg(not(target_arch = "wasm32"))]
            client: reqwest::Client::new(),
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

        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self.client
                .get(WEATHER_API_BASE_URL)
                .query(&params)
                .send()
                .await?;

            if response.status() != 200 {
                return Err(format!("API request failed with status: {}", response.status()).into());
            }

            let body = response.text().await?;
            let weather: WeatherResponse = serde_json::from_str(&body)?;

            // Print weather information
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

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{Request, RequestInit, Response};

            // Build URL with query parameters
            let mut url_with_params = WEATHER_API_BASE_URL.to_string();
            if !params.is_empty() {
                url_with_params.push('?');
                let query_params: Vec<String> = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                url_with_params.push_str(&query_params.join("&"));
            }

            let mut opts = RequestInit::new();
            opts.method("GET");

            let request = Request::new_with_str_and_init(&url_with_params, &opts)
                .map_err(|e| format!("Failed to create request: {:?}", e))?;

            let window = web_sys::window()
                .ok_or("No window object available")?;

            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|e| format!("Fetch failed: {:?}", e))?;

            let resp: Response = resp_value.dyn_into()
                .map_err(|_| "Failed to convert response")?;

            let status = resp.status();

            if status != 200 {
                return Err(format!("API request failed with status: {}", status).into());
            }

            let text_promise = resp.text()
                .map_err(|e| format!("Failed to get text promise: {:?}", e))?;

            let text = JsFuture::from(text_promise)
                .await
                .map_err(|e| format!("Failed to get text: {:?}", e))?;

            let body = text.as_string()
                .unwrap_or_else(|| "Failed to convert response to string".to_string());

            let weather: WeatherResponse = serde_json::from_str(&body)?;

            // Print weather information
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