use std::env;
use clap::Parser;
use dotenv::dotenv;

use openweathermap_lib::location::LocationClient;
use openweathermap_lib::weather::WeatherClient;

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
    no_display: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv() {
        Ok(_) => println!("Loaded .env file successfully"),
        Err(e) => println!("Warning: Could not load .env file: {}", e),
    }
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    match env::var(OPENWEATHERMAP_API_KEY) {
        Ok(api_key) => {
            let location_client = LocationClient::new(args.zip.clone(), args.country.clone(), api_key.clone());
            let location = location_client.get_location().await?;

            if !args.no_display {
                println!("{}", location.to_string());
            }

            let weather_client = WeatherClient::new(location, args.units.clone(), api_key.clone());
            let weather_response = weather_client.get_current_weather().await?;

            if !args.no_display {
                println!("{}", weather_response.detailed_display(args.units.clone()));
            }
        }
        Err(env::VarError::NotPresent) => handle_no_api_key_set(),
        Err(env::VarError::NotUnicode(_)) => handle_invalid_utf8(),
    }

    Ok(())
}

fn handle_no_api_key_set() {
    println!("{} is not set", OPENWEATHERMAP_API_KEY);
}

fn handle_invalid_utf8(){
    println!("{} contains invalid UTF-8", OPENWEATHERMAP_API_KEY);
}
