use super::types::Location;
use std::collections::HashMap;

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
    #[cfg(not(target_arch = "wasm32"))]
    client: reqwest::Client,
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
            #[cfg(not(target_arch = "wasm32"))]
            client: reqwest::Client::new(),
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
        let mut params = HashMap::new();
        params.insert("zip".to_string(), format!("{},{}", self.zip, self.country));
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
            println!("🌐 Location Endpoint: {}?{}", GEOCODING_API_BASE_URL, safe_params);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self.client
                .get(GEOCODING_API_BASE_URL)
                .query(&params)
                .send()
                .await?;

            if response.status() != 200 {
                return Err(format!("API request failed with status: {}", response.status()).into());
            }

            let body = response.text().await?;
            let location: Location = serde_json::from_str(&body)?;

            if debug {
                println!("zip: {}", location.zip);
                println!("name: {}", location.name);
                println!("country: {}", location.country);
                println!("lat: {}", location.lat);
                println!("lon: {}", location.lon);
            }

            Ok(location)
        }

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{Request, RequestInit, Response};

            // Build URL with query parameters
            let mut url_with_params = GEOCODING_API_BASE_URL.to_string();
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

            let location: Location = serde_json::from_str(&body)?;

            if debug {
                println!("zip: {}", location.zip);
                println!("name: {}", location.name);
                println!("country: {}", location.country);
                println!("lat: {}", location.lat);
                println!("lon: {}", location.lon);
            }

            Ok(location)
        }
    }
}