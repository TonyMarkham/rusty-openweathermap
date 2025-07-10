//! # Location Module
//!
//! This module provides functionality for geocoding ZIP codes to geographic coordinates
//! using the OpenWeatherMap Geocoding API.
//!
//! ## Components
//!
//! - `LocationClient`: Client for making geocoding API requests
//! - `Location`: Data structure representing geographic location information

pub mod types;
pub mod client;

pub use types::*;
pub use client::LocationClient;
