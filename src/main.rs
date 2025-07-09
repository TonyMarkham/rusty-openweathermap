use std::env;
use clap::Parser;
use weather_cli::{Location, location::LocationClient, weather::WeatherClient};

const OPENWEATHERMAP_API_KEY: &str = "OPENWEATHERMAP_API_KEY";


#[derive(Parser)]
#[command(name = "weather_cli")]
#[command(about = "Get weather information by ZIP code")]
#[command(version = "0.1.0")]
struct CliArgs {
    /// ZIP code (e.g., N7L)
    #[arg(short, long)]
    zip: String,

    /// Country code (e.g., CA, US)
    #[arg(short, long, default_value = "CA")]
    country: String,

    /// Units of measurement. `standard`, `metric` and `imperial` units are available.
    #[arg(short, long, default_value = "standard")]
    units: String,

    /// Show Location Debug Details
    #[arg(short, long, default_value = "false")]
    location_debug: bool,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match env::var(OPENWEATHERMAP_API_KEY) {
        Ok(value) => {
            if let Err(e) = get_location(value, args).await {
                eprintln!("Error: {}", e);
            }
        },
        Err(env::VarError::NotPresent) => handle_not_present(),
        Err(env::VarError::NotUnicode(_)) => handle_invalid_utf8(),
    }
}

fn handle_not_present() {
    println!("{} is not set", OPENWEATHERMAP_API_KEY);
}

fn handle_invalid_utf8() {
    println!("{} contains invalid UTF-8", OPENWEATHERMAP_API_KEY);
}

async fn get_location(api_key: String, args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let location_client = LocationClient::new(args.zip, args.country, api_key.clone());
    match location_client.get_location().await {
        Ok(location) => {
            if args.location_debug {
                println!("zip: {}", location.zip);
                println!("name: {}", location.name);
                println!("country: {}", location.country);
                println!("lat: {}", location.lat);
                println!("lon: {}", location.lon);
            }

            get_current_weather(location, args.units, api_key).await?;
        },
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    Ok(())
}

async fn get_current_weather(location: Location, units: String, api_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let weather_client = WeatherClient::new(location, &units, api_key.clone());
    match weather_client.get_current_weather().await {
        Ok(weather) => {
            println!("🌤️ Weather in {}", weather.name);
            println!("📍 Coordinates: ({}, {})", weather.coord.lat, weather.coord.lon);

            let temp_display = get_temperature_display(weather.main.temp, &units);
            println!("🌡️ Temperature: {}", temp_display);

            let wind_display = get_speed_display(weather.wind.speed, &units);
            println!("💨 Wind: {} at {}°", wind_display, weather.wind.deg);

            println!("☁️ Clouds: {}%", weather.clouds.all);

            if let Some(weather_info) = weather.weather.first() {
                println!("🌈 Conditions: {} ({})", weather_info.main, weather_info.description);
            }
        },
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    Ok(())
}

fn get_temperature_display(temp: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1}°C", temp),
        "imperial" => format!("{:.1}°F", temp),
        "standard" | _ => format!("{:.1}°K", temp),
    }
}

fn get_speed_display(speed: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1} m/s", speed),
        "imperial" => format!("{:.1} mph", speed),
        "standard" | _ => format!("{:.1} m/s", speed),
    }
}
