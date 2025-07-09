use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub zip: String,
}
