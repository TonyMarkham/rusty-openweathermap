use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    /// ZIP or postal code
    pub zip: String,
    /// City or locality name
    pub name: String,
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Two-letter country code (ISO 3166-1 alpha-2)
    pub country: String,
}

impl Location {
    pub fn detailed_display(&self) -> String {
        format!(
            r#"name: [{}]
country: [{}]
zip: [{}]
lat: [{}]
lon: [{}]"#,
            self.name,
            self.country,
            self.zip,
            self.lat,
            self.lon
        )
    }
}
