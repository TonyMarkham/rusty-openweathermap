use serde::{Deserialize, Serialize};

/// Represents a geographic location with coordinates and address information.
///
/// This structure contains all the essential information about a location
/// as returned by the OpenWeatherMap Geocoding API.
///
/// # Fields
///
/// * `zip` - The ZIP/postal code of the location
/// * `name` - The name of the city or locality
/// * `lat` - Latitude coordinate (decimal degrees)
/// * `lon` - Longitude coordinate (decimal degrees)
/// * `country` - Two-letter country code (ISO 3166-1 alpha-2)
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
