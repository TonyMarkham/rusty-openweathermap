use super::types::Location;

// https://api.openweathermap.org/geo/1.0/zip?zip=N7L,CA&appid={api_key}

const GEOCODING_API_BASE_URL: &str = "https://api.openweathermap.org/geo/1.0/zip";

/// A client for interacting with the OpenWeatherMap Geocoding API.
///
/// This client allows you to convert ZIP codes and country combinations
/// into geographic coordinates and location information.
pub struct LocationClient {
    api_key: String,
    zip: String,
    country: String,
}

impl LocationClient {
    /// Creates a new LocationClient instance.
    ///
    /// # Arguments
    ///
    /// * `zip` - The ZIP or postal code to geocode
    /// * `country` - Two-letter country code (e.g., "CA", "US")
    /// * `api_key` - Your OpenWeatherMap API key
    ///
    /// # Returns
    ///
    /// A new `LocationClient` instance configured with the provided parameters.
    pub fn new(zip: String, country: String, api_key: String) -> Self {
        Self {
            zip,
            country,
            api_key,
        }
    }

    /// Retrieves location information for the configured ZIP code and country.
    ///
    /// This method makes an asynchronous HTTP request to the OpenWeatherMap
    /// Geocoding API to convert the ZIP code and country into detailed
    /// location information including coordinates.
    ///
    /// # Arguments
    ///
    /// * `debug` - If true, prints debug information including the API endpoint URL
    ///   and the returned location data
    ///
    /// # Returns
    ///
    /// * `Ok(Location)` - On success, returns a Location struct with the geocoded information
    /// * `Err(Box<dyn std::error::Error>)` - On failure, returns an error describing what went wrong
    ///
    /// # Errors
    ///
    /// This method can fail for several reasons:
    /// - Network connectivity issues
    /// - Invalid API key
    /// - Invalid ZIP code or country combination
    /// - API rate limits exceeded
    /// - JSON parsing errors
    pub async fn get_location(&self, debug: bool) -> Result<Location, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request = client
            .get(GEOCODING_API_BASE_URL)
            .query(&[
                ("zip", &format!("{},{}", self.zip, self.country)),
                ("appid", &self.api_key),
            ])
            .build()?;

        if debug{
            // Hide the API key when printing
            let url_string = request.url().to_string();
            let safe_url = url_string.replace(&self.api_key, "{api_key}");
            println!("🌐 Location Endpoint: {}", safe_url);
        }

        let response = client.execute(request).await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let location: Location = response.json().await?;

        if debug{
            println!("zip: {}", location.zip);
            println!("name: {}", location.name);
            println!("country: {}", location.country);
            println!("lat: {}", location.lat);
            println!("lon: {}", location.lon);
        }

        Ok(location)
    }
}
