use std::env;
use clap::Parser;
use dotenv::dotenv;
use weather_cli::{location::LocationClient, weather::WeatherClient};

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
    print_debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = CliArgs::parse();

    match env::var(OPENWEATHERMAP_API_KEY) {
        Ok(api_key) => {
            let location_client = LocationClient::new(args.zip.clone(), args.country.clone(), api_key.clone());
            let location = location_client.get_location(args.print_debug).await?;

            let weather_client = WeatherClient::new(location, args.units.clone(), api_key.clone());
            weather_client.get_current_weather(args.units.clone(), args.print_debug.clone()).await?;
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
