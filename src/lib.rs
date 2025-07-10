//! # Weather CLI Library
//!
//! A library for fetching weather information and location data from OpenWeatherMap API.
//!
//! This library provides two main modules:
//! - `location`: For geocoding ZIP codes to coordinates
//! - `weather`: For fetching current weather information


pub mod weather;
pub mod location;

// Re-export commonly used types
pub use weather::{WeatherResponse, WeatherClient};
pub use location::{Location};
