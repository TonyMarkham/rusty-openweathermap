# Weather App

A Rust-based application that provides current weather information for a specified location using OpenWeatherMap API. The application supports both CLI and web-based interfaces, allowing users to access weather data through their preferred method.

## Features

- 🌍 Get weather data by ZIP code and country
- 🌡️ Support for different units of measurement (standard, metric, imperial)
- 🔍 Detailed weather information including temperature, wind, clouds, and conditions
- 🔒 Secure API key handling through environment variables
- 🧩 Modular architecture for easy extension and maintenance
- 🌐 WebAssembly (WASM) support for running in browsers
- 🖥️ Modern web interface for easy interaction with the weather service
- 🌈 Responsive design that works on desktop and mobile devices

## Installation

### Prerequisites

- Rust and Cargo (1.88.0 or newer)
- OpenWeatherMap API key (obtain from [OpenWeatherMap](https://openweathermap.org/api))
- Node.js and npm (for web interface only)

### Common Setup

1. Clone the repository:

```bash
git clone https://github.com/TonyMarkham/rusty-openweathermap.git
cd rusty-openweathermap
```

2. Create a `.env` file in the project root with your API key:

```
OPENWEATHERMAP_API_KEY=your_api_key_here
```

### CLI Setup

Build the command-line interface tool:

```bash
cargo build --release -p weather_cli
```

The executable will be available at `./target/release/weather_cli`

### Web GUI Setup

1. Install Node.js dependencies:

```bash
cd weather_web
npm install
```

2. Install wasm-pack if you don't have it:

```bash
cargo install wasm-pack
```

3. Build the WebAssembly module and prepare the web interface:

```bash
# Using npm script (recommended)
cd weather_web
npm run build-wasm-release

# Or manually
wasm-pack build weather_lib --target web --out-dir weather_web/web-gui/pkg --release --scope weather
```

## Usage

### Command Line Interface

Run the CLI with your ZIP code:

```bash
./target/release/weather_cli --zip N7L --country CA
```

### Web Interface

The application includes a web-based GUI that can be accessed through a browser:

1. Build the WebAssembly module:
```bash
cd weather_web
npm run build-wasm
```

2. Start the local web server:
```bash
cd weather_web
npm run serve
```

3. Navigate to `http://localhost:8000` in your browser

4. Enter your ZIP/postal code, country, preferred units, and your OpenWeatherMap API key to get weather information

### Command-line Options

- `-z, --zip <ZIP>`: ZIP code (required)
- `-c, --country <COUNTRY>`: Country code (default: "CA")
- `-u, --units <UNITS>`: Units of measurement (default: "standard")
  - `standard`: Kelvin for temperature, m/s for wind
  - `metric`: Celsius for temperature, m/s for wind
  - `imperial`: Fahrenheit for temperature, mph for wind
- `-n, --no-display`: Hide detailed output (default: false)

### Examples

Get weather for a US ZIP code in imperial units:

```bash
./target/release/weather_cli --zip 90210 --country US --units imperial
```

Get weather for a Canadian postal code in metric units with full output:

```bash
./target/release/weather_cli --zip N7L --country CA --units metric
```

## Project Structure

```
├── weather_lib/            # Core library code
│   ├── src/                # Library source code
│   │   ├── location/       # Location-related modules
│   │   │   ├── client.rs   # Client for geocoding API
│   │   │   ├── mod.rs      # Module exports
│   │   │   ├── types.rs    # Location data structures
│   │   │   └── wasm.rs     # WebAssembly bindings for location
│   │   ├── weather/        # Weather-related modules
│   │   │   ├── client.rs   # Client for weather API
│   │   │   ├── mod.rs      # Module exports
│   │   │   └── types.rs    # Weather data structures
│   │   └── lib.rs          # Library exports
│   └── Cargo.toml          # Library dependencies
├── weather_cli/            # Command-line interface
│   ├── src/                # CLI source code
│   │   └── main.rs         # CLI entry point and argument parsing
│   └── Cargo.toml          # CLI dependencies
├── weather_web/            # Web interface
│   ├── web-gui/            # Web interface files
│   │   ├── index.html      # Main HTML page
│   │   ├── style.css       # Styling for the web interface
│   │   ├── script.js       # JavaScript for web interface
│   │   └── pkg/            # WebAssembly compiled output
│   ├── package.json        # Node.js dependencies and scripts
│   └── package-lock.json   # Lock file for package versions
├── Cargo.toml              # Workspace configuration
├── Makefile.toml           # Build tasks
└── .env                    # Environment variables (not in repo)
```

## API Integration

This tool integrates with two OpenWeatherMap APIs:

1. Geocoding API: Converts ZIP/postal codes to coordinates
   - Endpoint: `https://api.openweathermap.org/geo/1.0/zip`

2. Current Weather API: Provides weather data for coordinates
   - Endpoint: `https://api.openweathermap.org/data/2.5/weather`

## Dependencies

- `clap`: Command-line argument parsing (v4.5.41)
- `reqwest`: HTTP client for API requests (v0.12.22)
- `tokio`: Async runtime (v1.46.1)
- `serde`: JSON serialization/deserialization (v1.0.219)
- `serde_json`: JSON support (v1.0.140)
- `urlencoding`: URL encoding utilities (v2.1.3)
- `dotenv`: Environment variable loading (v0.15.0)
- `wasm-bindgen`: WebAssembly bindings for JavaScript interoperability (v0.2.100)
- `wasm-bindgen-futures`: Future support for WASM (v0.4.50)
- `console_error_panic_hook`: Better error handling in WASM (v0.1.7)
- `serde-wasm-bindgen`: Serialization support for WASM (v0.6.5)

## Development

### Building the CLI

```bash
cargo build -p weather_cli
```

### Running in Debug Mode

```bash
cargo run -p weather_cli -- --zip N7L --country CA
```

### Running Tests

```bash
cargo test -p weather_lib
```

### Building for WebAssembly

```bash
# Using npm script (recommended)
cd weather_web
npm run build-wasm

# Or manually with wasm-pack
wasm-pack build ../weather_lib --target web --out-dir ../weather_web/web-gui/pkg --dev --scope weather
```

The above commands will:
1. Compile your Rust code to WebAssembly
2. Generate JavaScript bindings
3. Create necessary package.json and TypeScript definition files
4. Output everything to the `web-gui/pkg/` directory

### Developing the Web Interface

```bash
cd weather_web

# Build WASM and start local server in one command
npm run dev

# Or start server only (if WASM is already built)
npm run serve
```

This will start a local development server at http://localhost:8000 where you can test the web interface.

## Makefile.toml Tasks

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) to simplify common development tasks. To use these tasks, install cargo-make:

```bash
cargo install cargo-make
```

Then you can run the following tasks:

### Build WebAssembly (Development)

```bash
cargo make build-wasm
```

Builds the WebAssembly module in development mode with debugging information. This task compiles the `weather_lib` crate to WebAssembly and outputs it to the `../weather_web/web-gui/pkg` directory with the `weather` scope.

### Build WebAssembly (Release)

```bash
cargo make build-wasm-release
```

Builds the WebAssembly module in release mode with optimizations. Similar to `build-wasm` but produces a smaller, faster build suitable for production use.

### Start Development Server

```bash
cargo make serve
```

Starts a Python HTTP server on port 8000 serving the web GUI. This task makes the web interface accessible at `http://localhost:8000`.

### Clean Project

```bash
cargo make clean
```

Removes build artifacts including the WebAssembly package directory and the Rust target directory.

### Development Workflow

```bash
cargo make dev
```

Combines the `build-wasm` and `serve` tasks in sequence. This is the recommended command during development as it builds the WebAssembly module and immediately starts the web server.

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
import init, { getLocation, getCurrentWeather } from './pkg/weather_lib.js';

async function run() {
  // Initialize the WASM module
  await init();

  try {
    // Get location data first
    const location = await getLocation('N7L', 'CA', 'your_api_key');

    // Then get weather data for that location
    const weatherData = await getCurrentWeather(location, 'metric', 'your_api_key');

    // Display the results
    console.log('Location:', location);
    console.log('Weather:', weatherData);
  } catch (error) {
    console.error('Error fetching weather data:', error);
  }
}

run();
```

The web GUI in this project provides a complete implementation example of how to use the WebAssembly module with a form-based interface.

## License

This project is licensed under the terms found in the LICENSE file in the repository.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

*Note: This tool requires a valid OpenWeatherMap API key to function properly.*

---
*Last updated: July 12, 2025*
