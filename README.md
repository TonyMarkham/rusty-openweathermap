# Weather App

A Rust-based application that provides current weather information for a specified location using OpenWeatherMap API. The application supports both CLI and web-based interfaces, allowing users to access weather data through their preferred method.

## Features

- 🌍 Get weather data by ZIP code and country
- 🌡️ Support for different units of measurement (standard, metric, imperial)
- 🔍 Detailed weather information including temperature, wind, clouds, and conditions
- 🔒 Secure API key handling through environment variables
- 🧩 Modular architecture with a core Rust library
- 🌐 WebAssembly (WASM) support for running in browsers
- 🖥️ Command-line interface for quick access
- 🌈 Modern web interface with responsive design

## Installation

### Prerequisites

- Rust and Cargo (1.88.0 or newer)
- [OpenWeatherMap API key](https://openweathermap.org/api)
- cargo-make (optional, for running Makefile.toml tasks)
- Python (for running a local web server)

### Cloning the Repository

```bash
# Clone the repository
git clone https://github.com/TonyMarkham/rusty-openweathermap.git
cd rusty-openweathermap

# Create a .env file with your API key
echo "OPENWEATHERMAP_API_KEY=your_api_key_here" > .env
```

## Project Structure

```
├── weather_cli/               # Command-line interface application
│   ├── src/
│   │   └── main.rs           # CLI entry point and argument parsing
│   └── Cargo.toml            # CLI dependencies
│
├── weather_web/              # Web interface application
│   ├── src/                  # Rust source code for WASM module
│   ├── web-gui/              # Web interface files
│   │   ├── index.html        # Main HTML page
│   │   ├── style.css         # Styling for the web interface
│   │   ├── script.js         # JavaScript for the web interface
│   │   └── pkg/              # Output directory for WASM compilation
│   └── Cargo.toml            # Web app dependencies
│
├── Cargo.toml                # Workspace configuration
├── Cargo.lock                # Dependency lock file
├── Makefile.toml             # Build tasks for cargo-make
├── .env                      # Environment variables (not in repo)
└── .gitignore                # Git ignore file
```

This project uses OpenWeatherMap's API library (`openweathermap_lib`) as a dependency to fetch weather data. The code is organized into two main components:

1. **CLI Application**: A command-line tool for quickly fetching weather data
2. **Web Application**: A browser-based interface using WebAssembly

## Using Makefile.toml Tasks

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for managing build tasks. To use these tasks, first install cargo-make:

```bash
cargo install cargo-make
```

Then you can run the following tasks:

### Building the Development Version

```bash
cargo make build-dev
```

Builds the CLI application in development mode.

### Building WebAssembly for the Web GUI

```bash
cargo make build-web-wasm-dev  # Build WASM in development mode
cargo make build-wasm-dev       # Build WASM using wasm-pack (development)
cargo make build-wasm-release   # Build WASM using wasm-pack (release/optimized)
```

### Starting a Development Server

```bash
cargo make serve
```

Starts a Python HTTP server on port 8000 serving the web GUI files from `weather_web/web-gui`.

### Cleaning the Project

```bash
cargo make clean
```

Removes build artifacts including the web-gui/pkg directory and target directory.

## Running the CLI

The CLI application provides a simple interface for fetching weather data from the command line.

### Building the CLI

```bash
cargo build -p weather_cli
```

This will create an executable at `target/debug/weather_cli`.

### CLI Usage

```bash
# Get weather for a specific ZIP code and country
./target/debug/weather_cli --zip N7L --country CA

# Using different units (standard, metric, imperial)
./target/debug/weather_cli --zip N7L --country CA --units metric

# Get raw data without display formatting
./target/debug/weather_cli --zip N7L --country CA --no-display
```

### Command-line Options

- `-z, --zip <ZIP>`: ZIP code (required)
- `-c, --country <COUNTRY>`: Country code (default: "CA")
- `-u, --units <UNITS>`: Units of measurement (default: "standard")
- `--no-display`: Hide detailed output (default: false)

## Running the Web GUI

The web-based interface allows you to interact with the weather application through a browser.

### Building the WebAssembly Module

```bash
# Build the WASM module for development
cargo make run-dev

# Or build optimized version for production
cargo make build-wasm-release
```

This compiles the Rust code to WebAssembly and creates the necessary JavaScript bindings in the `weather_web/web-gui/pkg` directory.

### Starting the Web Server

```bash
cargo make serve
```

This starts a Python HTTP server on port 8000 serving the web GUI files.

### Accessing the Web Interface

Open your browser and navigate to:
```
http://localhost:8000
```

The web interface allows you to:

1. Enter a ZIP/postal code
2. Select a country code
3. Choose units of measurement (standard, metric, imperial)
4. View detailed weather information including:
   - Current temperature
   - Weather conditions with icon
   - Wind speed and direction
   - Humidity and pressure
   - Visibility
   - Sunrise and sunset times

## Dependencies

This project uses the following dependencies:

- **Rust Libraries**:
  - `tokio` (v1.46.1): Asynchronous runtime
  - `clap` (v4.5.41): Command-line argument parsing
  - `openweathermap_lib` (v0.1.0-pre.2): OpenWeatherMap API client
  - `dotenv` (v0.15.0): Environment variable loading
  - Various WebAssembly-related crates for the web interface

## Development

### Adding Features

To extend the application with additional features:

1. For weather data features, modify the OpenWeatherMap library integration
2. For CLI features, update the command-line argument parsing in `weather_cli/src/main.rs`
3. For web interface features, update both the Rust code in `weather_web/src` and the web files in `weather_web/web-gui`

### Testing

Run tests with:

```bash
cargo test
```

## Troubleshooting

- **API Key Issues**: Ensure your OpenWeatherMap API key is correctly set in the `.env` file
- **WASM Build Errors**: Make sure you have `wasm-pack` installed (`cargo install wasm-pack`)
- **Browser Compatibility**: The web interface works best in modern browsers that support WebAssembly

## License

This project is available under the terms of the MIT License. See the LICENSE file for details.

---

*This README was last updated: July 12, 2025*
