use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Location as InternalLocation, LocationInfo};

/// JavaScript-compatible location information
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmLocation {
    /// Location name
    pub name: String,
    /// Latitude coordinate
    pub lat: f64,
    /// Longitude coordinate
    pub lon: f64,
    /// Country code
    pub country: String,
    /// State or region (optional)
    pub state: Option<String>,
}

#[wasm_bindgen]
impl WasmLocation {
    /// Creates a new WasmLocation
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, lat: f64, lon: f64, country: String, state: Option<String>) -> WasmLocation {
        WasmLocation {
            name,
            lat,
            lon,
            country,
            state,
        }
    }

    /// Gets the location name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Gets the latitude
    #[wasm_bindgen(getter)]
    pub fn lat(&self) -> f64 {
        self.lat
    }

    /// Gets the longitude
    #[wasm_bindgen(getter)]
    pub fn lon(&self) -> f64 {
        self.lon
    }

    /// Gets the country code
    #[wasm_bindgen(getter)]
    pub fn country(&self) -> String {
        self.country.clone()
    }

    /// Gets the state (optional)
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> Option<String> {
        self.state.clone()
    }
}

impl LocationInfo for WasmLocation {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn lat(&self) -> f64 {
        self.lat
    }
    
    fn lon(&self) -> f64 {
        self.lon
    }
    
    fn country(&self) -> &str {
        &self.country
    }
}

impl From<InternalLocation> for WasmLocation {
    fn from(loc: InternalLocation) -> Self {
        WasmLocation {
            name: loc.name,
            lat: loc.lat,
            lon: loc.lon,
            country: loc.country,
            state: None, // Internal location doesn't have state field
        }
    }
}

impl From<WasmLocation> for InternalLocation {
    fn from(loc: WasmLocation) -> Self {
        InternalLocation {
            zip: String::new(), // WASM location doesn't have zip field
            name: loc.name,
            lat: loc.lat,
            lon: loc.lon,
            country: loc.country,
        }
    }
}