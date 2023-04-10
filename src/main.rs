use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use chrono_systemd_time::{ parse_timestamp_tz, InvalidTimestamp};
use chrono;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure credentials
    Configure(ConfigureArgs),
    Get(GetArgs),
}

#[derive(Args)]
struct ConfigureArgs {
    /// Provider name
    provider: String,

    /// To update api key
    #[arg(long)]
    api: Option<String>,

    /// To update address for which weahter information should be received 
    #[arg(short, long)]
    address: Option<String>
}

#[derive(Args)]
struct GetArgs {
    #[arg(short, long)]
    address: Option<String>,

    #[arg(short, long, default_value_t = String::from("now"))]
    time: String,
    // time: chrono::DateTime<chrono::Utc>,
}

fn parse_time(arg: &str) -> Result<chrono::DateTime<chrono::Utc>, InvalidTimestamp> {
    let seconds = parse_timestamp_tz(arg, chrono::Utc)?;
    Ok(seconds)
}

#[derive(Serialize)]
struct ProviderConfiguration {
    api_key: String
}

#[derive(Serialize)]
struct Providers {
    openweather: Option<ProviderConfiguration>,
    weatherapi: Option<ProviderConfiguration>,
}

// #[derive(Serialize)]
// enum APIProviders {
//     OpenWeatherMap(String),
//     WeatherAPI(String),
// }

#[derive(Serialize)]
struct AppConfiguration {
    address: Option<String>,
    preferred_provider: Option<String>,
    providers: Providers
}

fn main() {
    let cli = CLI::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Configure(cfg)) => {
            println!("Provider: {:?}", cfg.provider);

            if let Some(api) = cfg.api.as_deref() {
                println!("Value for api: {api}");
            }

            if let Some(address) = cfg.address.as_deref() {
                println!("Value for address: {address}");
            }
        }
        Some(Commands::Get(cfg)) => {
            println!("Address: {:?}", cfg.address);

            let parsed_time = parse_time(cfg.time.as_str()).unwrap();

            println!("Time: {:?}", parsed_time);
        }
        _ => {
            println!("Hello, world!");
        }
    }
}