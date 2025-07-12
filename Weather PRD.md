# Product Requirements Document (PRD)

## Project: Weather Application (Rust & WASM)

---

## Purpose & Scope

Develop a cross-platform weather information solution that queries the OpenWeatherMap API for current weather data by ZIP code or location.  
This product includes:

- A Rust library for API interactions
- A Command-Line Interface (CLI)
- A Web GUI built using Rust, compiled to WebAssembly (WASM)  
  All communications with OpenWeatherMap must be encapsulated within the Rust library.

---

## User Stories

### End Users
- As a user, I want to fetch current weather data for a given ZIP code and country, so that I can see the latest local weather.
- As a CLI user, I want to call a command with my ZIP code and country code, and receive the current weather details.
- As a web user, I want to enter a ZIP code and country, view current weather (and optionally details) via the web GUI in my browser.
- As a developer, I want a reusable Rust library that abstracts all OpenWeatherMap API details.

---

## Functional Requirements

### General
- All business logic for weather querying and data structures must reside in a Rust library (`weather_lib`).
- The Rust library must expose necessary methods for:
    - Location lookup by ZIP/country
    - Fetching current weather data
- The solution **must never call the OpenWeatherMap API except through the library**.

### CLI Implementation
- CLI must be implemented in Rust.
- Users provide:
    - `--zip` (ZIP code) [required]
    - `--country` (country code, default `CA`)
    - `--units` (“standard”, “metric”, or “imperial”; default: “standard”)
    - `--no-display` (flag: do not print output; default: false)
- Uses the library to fetch and display location and weather details.
- Gracefully handle missing or invalid API keys via informative CLI error messages.
- Support `.env` file loading for the API key.

### Web GUI (WASM)
- Implemented in Rust, compiled to WebAssembly (WASM) targeting web browsers.
- The GUI must allow:
    - Input of ZIP code and country code
    - Unit selection
    - On submit: call Rust (via WASM) to fetch weather data, display results or user-friendly errors.
- All browser API interactions must occur via WASM → Rust library, **never directly from JS to OpenWeatherMap**.

### Error Handling
- Provide meaningful, user-facing error output for:
    - Missing/invalid/unset API keys
    - Network failures
    - Invalid or unknown ZIP/country codes
    - Malformed responses

---

## Non-Functional Requirements

- **Language**: Rust 1.88.0 (core); WASM (for Web GUI); CLI (Rust)
- **Rust library** encapsulates all OpenWeatherMap and model logic, is reusable by both CLI and WASM web frontends.
- **Dependency Management**
    - Rust: Use `Cargo.toml`
    - JS: Use `npm` with `rimraf` as noted; all JS should be minimal, only for WASM interop/boilerplate.
    - WebAssembly: Use `wasm-bindgen`, `wasm-bindgen-futures`
- **Platform**: Works on Windows 11; CLI and Web
- **OpenWeatherMap API Key**: Sourced from environment variable (`OPENWEATHERMAP_API_KEY`), not hardcoded.

---

## Technical Constraints

- **OpenWeatherMap Logic**  
  All calls and HTTP interaction with OpenWeatherMap must be implemented in a dedicated Rust library crate.
- **API Key Security**
    - The API key must **never** be exposed client-side or in JS.
- **Reusability**  
  No duplication of OpenWeatherMap model or request/response structures outside the library; both frontend and CLI depend on a single interface.
- **Build & Deployment**
    - CLI: standard Rust build
    - Web: build Rust to WASM, minimal JS glue code

---

## Library Interface Requirements

- Expose Rust structs for Weather, Location, etc. (modeled after OpenWeatherMap responses)
- Provide async functions for:
    - Resolving coordinates by ZIP/country
    - Fetching current weather by coordinates, ZIP, or location object
- All HTTP interactions must use `reqwest` (async).
- Ensure seamless WASM-compatibility (via `wasm-bindgen`).

---

## CLI Requirements

- Use `clap` for command-line argument parsing
- Use `dotenv` to load environment variables from `.env`
- Output should be human-friendly by default; offer a debug mode for raw or structured output

---

## WASM Web GUI Requirements

- Provide a responsive web interface for:
    - Entering ZIP code & country
    - Selecting units
    - Viewing formatted current weather
- Use JS only for initializing/loading the WASM module, and passing user input/results between WASM and browser

---

## Dependencies

- Rust: `reqwest`, `serde`, `serde_json`, `tokio`, `clap`, `dotenv`, `wasm-bindgen`, `wasm-bindgen-futures`
- JS: `rimraf` via `npm`

---

## Quality

- All HTTP/networking code is robustly error-checked
- Typed structs for all data
- Meaningful error messages throughout
- Modular, well-documented Rust source

---

## Out of Scope

- Forecast data (only current conditions required)
- UI theming/css beyond basic usability
- Mobile apps or packaging

---

## Acceptance Criteria

- [ ] Weather queries only hit OpenWeatherMap via Rust lib
- [ ] CLI works for any given (ZIP, country, units)
- [ ] Web GUI works in a modern browser, via WASM
- [ ] Environment variable for API key loads from `.env` or shell
- [ ] Proper error messages for all failure cases
- [ ] No API key or HTTP logic in JS or frontend code