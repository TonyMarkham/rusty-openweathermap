use super::types::WeatherResponse;
use crate::location::Location;

// https://api.openweathermap.org/data/2.5/weather?lat=42.44209&lon=-82.1993&appid={api_key}

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

/// A client for interacting with the OpenWeatherMap Current Weather API.
///
/// This client allows you to fetch current weather information for a specific
/// location using geographic coordinates. It supports different unit systems
/// and provides formatted output for weather data.
pub struct WeatherClient {
    location: Location,
    units: String,
    api_key: String,
}

impl WeatherClient {
    /// Creates a new WeatherClient instance.
    ///
    /// # Arguments
    ///
    /// * `location` - The geographic location for which to fetch weather data
    /// * `units` - The unit system to use ("standard", "metric", or "imperial")
    /// * `api_key` - Your OpenWeatherMap API key
    ///
    /// # Returns
    ///
    /// A new `WeatherClient` instance configured with the provided parameters.
    ///
    /// # Unit Systems
    ///
    /// - `"standard"` - Kelvin for temperature, m/s for wind speed
    /// - `"metric"` - Celsius for temperature, m/s for wind speed
    /// - `"imperial"` - Fahrenheit for temperature, mph for wind speed
    pub fn new(location: Location, units: String, api_key: String) -> Self {
        Self {
            location,
            units: units.clone(),
            api_key,
        }
    }

    /// Fetches current weather information for the configured location.
    ///
    /// This method makes an asynchronous HTTP request to the OpenWeatherMap
    /// Current Weather API to get detailed weather information for the
    /// location's coordinates. It also prints formatted weather information
    /// to the console.
    ///
    /// # Arguments
    ///
    /// * `units` - The unit system to use for the API request and display formatting
    /// * `debug` - If true, prints debug information including the API endpoint URL
    ///
    /// # Returns
    ///
    /// * `Ok(WeatherResponse)` - On success, returns the complete weather data
    /// * `Err(Box<dyn std::error::Error>)` - On failure, returns an error describing what went wrong
    ///
    /// # Errors
    ///
    /// This method can fail for several reasons:
    /// - Network connectivity issues
    /// - Invalid API key
    /// - Invalid coordinates
    /// - API rate limits exceeded
    /// - JSON parsing errors
    ///
    /// # Console Output
    ///
    /// This method prints formatted weather information to the console, including:
    /// - Location name and coordinates
    /// - Current temperature (with appropriate unit symbol)
    /// - Wind speed and direction
    /// - Cloud coverage percentage
    /// - Weather conditions and description
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

/// Formats temperature values with appropriate unit symbols.
///
/// This function takes a temperature value and unit system string,
/// then returns a formatted string with the appropriate unit symbol.
///
/// # Arguments
///
/// * `temp` - The temperature value to format
/// * `units` - The unit system ("standard", "metric", or "imperial")
///
/// # Returns
///
/// A formatted string with the temperature and appropriate unit symbol:
/// - "metric": Returns temperature in Celsius (°C)
/// - "imperial": Returns temperature in Fahrenheit (°F)
/// - "standard" or any other value: Returns temperature in Kelvin (°K)
fn get_temperature_display(temp: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1}°C", temp),
        "imperial" => format!("{:.1}°F", temp),
        "standard" | _ => format!("{:.1}°K", temp),
    }
}

/// Formats wind speed values with appropriate unit symbols.
///
/// This function takes a wind speed value and unit system string,
/// then returns a formatted string with the appropriate unit symbol.
///
/// # Arguments
///
/// * `speed` - The wind speed value to format
/// * `units` - The unit system ("standard", "metric", or "imperial")
///
/// # Returns
///
/// A formatted string with the wind speed and appropriate unit symbol:
/// - "metric": Returns speed in meters per second (m/s)
/// - "imperial": Returns speed in miles per hour (mph)
/// - "standard" or any other value: Returns speed in meters per second (m/s)
fn get_speed_display(speed: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1} m/s", speed),
        "imperial" => format!("{:.1} mph", speed),
        "standard" | _ => format!("{:.1} m/s", speed),
    }
}
