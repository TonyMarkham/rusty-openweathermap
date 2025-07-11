use super::types::Location;

// https://api.openweathermap.org/geo/1.0/zip?zip=N7L,CA&appid={api_key}

const GEOCODING_API_BASE_URL: &str = "https://api.openweathermap.org/geo/1.0/zip";

pub struct LocationClient {
    client: reqwest::Client,
    api_key: String,
    zip: String,
    country: String,
}

impl LocationClient {
    pub fn new(zip: String, country: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            zip,
            country,
            api_key,
        }
    }

    pub async fn get_location(&self) -> Result<Location, Box<dyn std::error::Error>> {
        let zip = format!("{},{}", &self.zip, &self.country);

        let response = self.client
            .get(GEOCODING_API_BASE_URL)
            .query(&[
                ("zip", &zip),
                ("appid", &self.api_key)
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }

    pub fn detailed_display(&self) -> String {
        format!(
            r#"country: [{}] - zip: [{}]"#,
            self.country,
            self.zip,
        )
    }
}
