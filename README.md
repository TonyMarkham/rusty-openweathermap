# Weather CLI

A command-line tool written in Rust that provides current weather information for a specified location using OpenWeatherMap API. The application supports both CLI and WebAssembly environments.

## Features

- 🌍 Get weather data by ZIP code and country
- 🌡️ Support for different units of measurement (standard, metric, imperial)
- 🔍 Detailed weather information including temperature, wind, clouds, and conditions
- 🔒 Secure API key handling through environment variables
- 🧩 Modular architecture for easy extension and maintenance
- 🌐 WebAssembly (WASM) support for running in browsers

## Installation

### Prerequisites

- Rust and Cargo (1.88.0 or newer)
- OpenWeatherMap API key (obtain from [OpenWeatherMap](https://openweathermap.org/api))

### Setup

1. Clone the repository:

```bash
git clone https://github.com/TonyMarkham/rusty-openweathermap.git
cd rusty-openweathermap
```

2. Create a `.env` file in the project root with your API key:

```
OPENWEATHERMAP_API_KEY=your_api_key_here
```

3. Build the project:

```bash
cargo build --release
```

Or, if you want to build the WebAssembly version:

```bash
# Install wasm-pack if you don't have it
cargo install wasm-pack

# Build with wasm-pack for better browser compatibility
wasm-pack build --target web --features wasm
```

## Usage

Run the CLI with your ZIP code:

```bash
./target/release/weather-cli --zip N7L --country CA
```

### Command-line Options

- `-z, --zip <ZIP>`: ZIP code (required)
- `-c, --country <COUNTRY>`: Country code (default: "CA")
- `-u, --units <UNITS>`: Units of measurement (default: "standard")
  - `standard`: Kelvin for temperature, m/s for wind
  - `metric`: Celsius for temperature, m/s for wind
  - `imperial`: Fahrenheit for temperature, mph for wind
- `-l, --print-debug`: Show location debug details (default: false)

### Examples

Get weather for a US ZIP code in imperial units:

```bash
./target/release/weather-cli --zip 90210 --country US --units imperial
```

Get weather for a Canadian postal code in metric units with debug info:

```bash
./target/release/weather-cli --zip N7L --country CA --units metric --print-debug
```

## Project Structure

```
├── src/
│   ├── location/           # Location-related modules
│   │   ├── client.rs       # Client for geocoding API
│   │   ├── mod.rs          # Module exports
│   │   ├── types.rs        # Location data structures
│   │   └── wasm.rs         # WebAssembly bindings for location
│   ├── weather/            # Weather-related modules
│   │   ├── client.rs       # Client for weather API
│   │   ├── mod.rs          # Module exports
│   │   └── types.rs        # Weather data structures
│   ├── lib.rs              # Library exports
│   └── main.rs             # CLI entry point and argument parsing
├── Cargo.toml              # Project dependencies
└── .env                    # Environment variables (not in repo)
```

## API Integration

This tool integrates with two OpenWeatherMap APIs:

1. Geocoding API: Converts ZIP/postal codes to coordinates
   - Endpoint: `https://api.openweathermap.org/geo/1.0/zip`

2. Current Weather API: Provides weather data for coordinates
   - Endpoint: `https://api.openweathermap.org/data/2.5/weather`

## Dependencies

- `clap`: Command-line argument parsing
- `reqwest`: HTTP client for API requests
- `tokio`: Async runtime
- `serde`: JSON serialization/deserialization
- `dotenv`: Environment variable loading
- `wasm-bindgen`: WebAssembly bindings for JavaScript interoperability
- `wasm-bindgen-futures`: Future support for WASM
- `console_error_panic_hook`: Better error handling in WASM
- `serde-wasm-bindgen`: Serialization support for WASM

## Development

### Building

```bash
cargo build
```

### Running in Debug Mode

```bash
cargo run -- --zip N7L --country CA
```

### Running Tests

```bash
cargo test
```

### Building for WebAssembly

```bash
# Compile using wasm-pack for optimal results
wasm-pack build --target web --features wasm
```

The above command will:
1. Compile your Rust code to WebAssembly
2. Generate JavaScript bindings
3. Create necessary package.json and TypeScript definition files
4. Output everything to the `pkg/` directory

## WebAssembly Support

This application can be compiled to WebAssembly for use in web browsers:

- Supports the same core functionality as the CLI version
- Can be integrated into web applications
- Uses the browser's fetch API for HTTP requests
- Platform-agnostic implementation works across browsers

### WASM Build Target Options

- `--target web`: For direct use in browsers with ES modules (most platform-agnostic)
- `--target bundler`: For use with bundlers like webpack
- `--target nodejs`: For Node.js environments
- `--target no-modules`: For use with script tags

### Example WASM Usage

```javascript
// Import the WASM package (using ES modules)
import init, { getLocation, getCurrentWeather } from './pkg/weather.js';

async function run() {
  // Initialize the WASM module
  await init();

  // Get weather for a location
  const location = await getLocation('N7L', 'CA');
  const weatherData = await getCurrentWeather(location, 'metric');
  console.log(weatherData);
}

run();
```

## License

This project is licensed under the terms found in the LICENSE file in the repository.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

*Note: This tool requires a valid OpenWeatherMap API key to function properly.*
