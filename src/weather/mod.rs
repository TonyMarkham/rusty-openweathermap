//! # Weather Module
//!
//! This module provides functionality for fetching current weather information
//! using the OpenWeatherMap Current Weather API.
//!
//! ## Components
//!
//! - `WeatherClient`: Client for making weather API requests
//! - Weather data types: Various structures representing weather information
//! - `WeatherInfo`: Common trait for all weather response types

pub mod types;
pub mod client;

pub use types::*;
pub use client::WeatherClient;