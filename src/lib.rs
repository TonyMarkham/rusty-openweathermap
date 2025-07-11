//! # Weather CLI Library
//!
//! A library for fetching weather information and location data from OpenWeatherMap API.
//!
//! This library provides three main modules:
//! - `location`: For geocoding ZIP codes to coordinates
//! - `weather`: For fetching current weather information
//! - `wasm`: For WebAssembly bindings (when compiled for web)

pub mod weather;
pub mod location;
pub mod http_client;

// Re-export commonly used types
pub use weather::{WeatherResponse, WeatherClient};
pub use location::{Location, LocationClient};
pub use http_client::HttpClient;
