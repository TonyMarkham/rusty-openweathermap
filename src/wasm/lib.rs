pub mod types;
pub mod weather;
pub mod location;

// Only include wasm module for wasm32 target
#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export the WASM functions when building for wasm32
#[cfg(target_arch = "wasm32")]
pub use wasm::*;