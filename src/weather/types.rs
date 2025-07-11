use serde::{Deserialize, Serialize};

/// Common interface for weather response types
pub trait WeatherInfo {
    fn name(&self) -> &str;
    fn get_data(&self) -> String;
}

/// Represents coordinate information with latitude and longitude.
///
/// This structure contains geographic coordinates as returned by the
/// OpenWeatherMap Current Weather API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Latitude in decimal degrees
    pub lat: f64,
}

/// Represents weather condition information.
///
/// This structure contains descriptive information about current weather conditions
/// including condition codes and human-readable descriptions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
    /// Weather condition ID (internal OpenWeatherMap identifier)
    pub id: i32,
    /// Group of weather parameters (Rain, Snow, Clouds, etc.)
    pub main: String,
    /// Weather condition description (e.g., "light rain", "clear sky")
    pub description: String,
    /// Weather icon ID for displaying weather icons
    pub icon: String,
}

/// Represents main weather measurements.
///
/// This structure contains the primary weather metrics including temperature,
/// pressure, humidity, and thermal perception values.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Main {
    /// Current temperature
    pub temp: f64,
    /// Temperature perception by humans
    pub feels_like: f64,
    /// Minimum temperature at the moment (for large cities and urban areas)
    pub temp_min: f64,
    /// Maximum temperature at the moment (for large cities and urban areas)
    pub temp_max: f64,
    /// Atmospheric pressure in hPa
    pub pressure: i32,
    /// Humidity percentage
    pub humidity: i32,
    /// Atmospheric pressure on the sea level in hPa (optional)
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level in hPa (optional)
    pub grnd_level: Option<i32>,
}

/// Represents wind information.
///
/// This structure contains wind speed and direction measurements.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wind {
    /// Wind speed (units vary by API request: m/s for metric, mph for imperial)
    pub speed: f64,
    /// Wind direction in degrees (meteorological)
    pub deg: i32,
    /// Wind gust speed (optional, same units as speed)
    pub gust: Option<f64>,
}

/// Represents cloud coverage information.
///
/// This structure contains cloudiness percentage data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clouds {
    /// Cloudiness percentage (0-100%)
    pub all: i32,
}

/// Represents visibility information.
///
/// This structure contains visibility distance measurements.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visibility {
    /// Visibility distance in meters
    pub visibility: i32,
}

/// Represents system information from the weather API.
///
/// This structure contains metadata about the weather data source
/// and timing information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    /// Internal parameter for data source
    #[serde(rename = "type")]
    pub sys_type: Option<i32>,
    /// Internal parameter for data source
    pub id: Option<i32>,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Sunrise time in Unix timestamp UTC
    pub sunrise: i64,
    /// Sunset time in Unix timestamp UTC
    pub sunset: i64,
}

/// Represents the complete weather response from the OpenWeatherMap API.
///
/// This is the main structure that contains all weather information
/// for a specific location, including coordinates, current conditions,
/// temperature, wind, clouds, and system information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherResponse {
    /// Geographic coordinates of the location
    pub coord: Coord,
    /// Weather condition information (can be multiple conditions)
    pub weather: Vec<Weather>,
    /// Internal parameter for data source
    pub base: String,
    /// Main weather measurements (temperature, pressure, humidity, etc.)
    pub main: Main,
    /// Visibility information
    pub visibility: i32,
    /// Wind information
    pub wind: Wind,
    /// Cloud coverage information
    pub clouds: Clouds,
    /// Time of data calculation in Unix timestamp UTC
    pub dt: i64,
    /// System information (country, sunrise, sunset, etc.)
    pub sys: Sys,
    /// Timezone shift in seconds from UTC
    pub timezone: i32,
    /// City/location ID
    pub id: i64,
    /// City/location name
    pub name: String,
    /// Internal parameter for API response
    pub cod: i32,
}

impl WeatherInfo for WeatherResponse {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn get_data(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}