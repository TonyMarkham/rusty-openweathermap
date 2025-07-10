use super::types::Location;

// https://api.openweathermap.org/geo/1.0/zip?zip=N7L,CA&appid={api_key}

const GEOCODING_API_BASE_URL: &str = "https://api.openweathermap.org/geo/1.0/zip";

pub struct LocationClient {
    api_key: String,
    zip: String,
    country: String,
}

impl LocationClient {
    pub fn new(zip: String, country: String, api_key: String) -> Self {
        Self {
            zip,
            country,
            api_key,
        }
    }

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
