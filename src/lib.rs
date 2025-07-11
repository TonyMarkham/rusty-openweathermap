pub mod weather;
pub mod location;

// Re-export commonly used types
pub use weather::{WeatherResponse, WeatherClient};
pub use location::{Location, LocationClient};
