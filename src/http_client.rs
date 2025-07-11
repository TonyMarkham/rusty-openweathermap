//! HTTP client abstraction for both native and WASM targets

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct HttpClient {
    #[cfg(not(target_arch = "wasm32"))]
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub struct WasmError {
    message: String,
}

#[cfg(target_arch = "wasm32")]
impl WasmError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[cfg(target_arch = "wasm32")]
impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WASM Error: {}", self.message)
    }
}

#[cfg(target_arch = "wasm32")]
impl std::error::Error for WasmError {}

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for WasmError {
    fn from(js_value: wasm_bindgen::JsValue) -> Self {
        let message = if let Some(string) = js_value.as_string() {
            string
        } else {
            format!("{:?}", js_value)
        };
        WasmError::new(message)
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str, params: HashMap<String, String>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self.client
                .get(url)
                .query(&params)
                .send()
                .await?;

            let status = response.status().as_u16();
            let body = response.text().await?;

            Ok(HttpResponse { status, body })
        }

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{Request, RequestInit, Response};

            // Build URL with query parameters
            let mut url_with_params = url.to_string();
            if !params.is_empty() {
                url_with_params.push('?');
                let query_params: Vec<String> = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                url_with_params.push_str(&query_params.join("&"));
            }

            let mut opts = RequestInit::new();
            opts.method("GET");

            let request = Request::new_with_str_and_init(&url_with_params, &opts)
                .map_err(WasmError::from)?;

            let window = web_sys::window()
                .ok_or_else(|| WasmError::new("No window object available".to_string()))?;

            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(WasmError::from)?;

            let resp: Response = resp_value.dyn_into()
                .map_err(|_| WasmError::new("Failed to convert response".to_string()))?;

            let status = resp.status();

            let text_promise = resp.text()
                .map_err(WasmError::from)?;

            let text = JsFuture::from(text_promise)
                .await
                .map_err(WasmError::from)?;

            let body = text.as_string()
                .unwrap_or_else(|| "Failed to convert response to string".to_string());

            Ok(HttpResponse { status, body })
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}