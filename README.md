# Weather CLI

A command-line tool written in Rust that provides current weather information for a specified location using OpenWeatherMap API.

## Features

- 🌍 Get weather data by ZIP code and country
- 🌡️ Support for different units of measurement (standard, metric, imperial)
- 🔍 Detailed weather information including temperature, wind, clouds, and conditions
- 🔒 Secure API key handling through environment variables
- 🧩 Modular architecture for easy extension and maintenance

## Installation

### Prerequisites

- Rust and Cargo (1.88.0 or newer)
- OpenWeatherMap API key (obtain from [OpenWeatherMap](https://openweathermap.org/api))

### Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/weather_cli.git
cd weather_cli
```

2. Create a `.env` file in the project root with your API key:

```
OPENWEATHERMAP_API_KEY=your_api_key_here
```

3. Build the project:

```bash
cargo build --release
```

## Usage

Run the CLI with your ZIP code:

```bash
./target/release/weather_cli --zip N7L --country CA
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
./target/release/weather_cli --zip 90210 --country US --units imperial
```

Get weather for a Canadian postal code in metric units with debug info:

```bash
./target/release/weather_cli --zip N7L --country CA --units metric --print-debug
```

## Project Structure

```
├── src/
│   ├── location/           # Location-related modules
│   │   ├── client.rs       # Client for geocoding API
│   │   ├── mod.rs          # Module exports
│   │   └── types.rs        # Location data structures
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

## License

This project is licensed under the terms found in the LICENSE file in the repository.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

*Note: This tool requires a valid OpenWeatherMap API key to function properly.*
