use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::location::LocationClient;
use crate::weather::WeatherClient;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherRequest {
    pub zip: String,
    pub country: String,
    pub units: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub location: crate::types::Location,
    pub weather: String,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub async fn get_weather_data(request_json: &str) -> Result<String, JsValue> {
    console_log!("WASM function called with: {}", request_json);
    
    let request: WeatherRequest = serde_json::from_str(request_json)
        .map_err(|e| {
            console_log!("JSON parse error: {}", e);
            JsValue::from_str(&format!("Invalid request: {}", e))
        })?;

    console_log!("Parsed request: {:?}", request);

    match fetch_weather_internal(request).await {
        Ok(response) => {
            console_log!("Weather fetch successful");
            serde_json::to_string(&response)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(e) => {
            console_log!("Weather fetch error: {}", e);
            let error_response = WeatherResponse {
                location: crate::types::Location {
                    zip: String::new(),
                    name: String::new(),
                    lat: 0.0,
                    lon: 0.0,
                    country: String::new(),
                },
                weather: String::new(),
                error: Some(e),
            };
            serde_json::to_string(&error_response)
                .map_err(|e| JsValue::from_str(&format!("Error serialization failed: {}", e)))
        }
    }
}

async fn fetch_weather_internal(request: WeatherRequest) -> Result<WeatherResponse, String> {
    console_log!("Creating location client");
    
    let location_client = LocationClient::new(
        request.zip.clone(),
        request.country.clone(),
        request.api_key.clone(),
    );

    console_log!("Fetching location");
    let location = location_client
        .get_location()
        .await
        .map_err(|e| format!("Location error: {}", e))?;

    console_log!("Location found: {:?}", location);

    let weather_client = WeatherClient::new(
        location.clone(),
        request.units.clone(),
        request.api_key.clone(),
    );

    console_log!("Fetching weather");
    let weather_response = weather_client
        .get_current_weather()
        .await
        .map_err(|e| format!("Weather error: {}", e))?;

    console_log!("Weather fetch complete");

    Ok(WeatherResponse {
        location,
        weather: serde_json::to_string(&weather_response)
            .map_err(|e| format!("Weather serialization error: {}", e))?,
        error: None,
    })
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Weather WASM module initialized");
}