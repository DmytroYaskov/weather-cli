use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};
use chrono_systemd_time::{ parse_timestamp_tz, InvalidTimestamp};
use chrono;
use toml;
use std::io::Write;
use directories::ProjectDirs;
use std::fs;

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
    #[arg(short, long)]
    api: Option<String>,

    /// To update location for which weahter information should be received 
    #[arg(short, long)]
    location: Option<String>
}

#[derive(Args)]
struct GetArgs {
    #[arg(short, long)]
    location: Option<String>,

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
struct Location {
    lat: f32,
    lon: f32,
}

#[derive(Serialize)]
struct AppConfiguration {
    location: Option<Location>,
    preferred_provider: Option<String>,
    providers: Providers
}

#[derive(Serialize, Deserialize)]
struct GeoInfo {
    name: String
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

            if let Some(location) = cfg.location.as_deref() {
                println!("Value for address: {location}");
            }

            // set configuration dir
            let proj_dirs = ProjectDirs::from("", "",  "weather-cli").unwrap();

            let project_folder = proj_dirs.config_dir().to_str().unwrap();

            // check if dir is exist
            if !fs::metadata(project_folder).is_ok() {
                // If the folder doesn't exist, create it
                match fs::create_dir(project_folder) {
                    Ok(_) => println!("Folder created successfully."),
                    Err(e) => println!("Error creating folder: {:?}", e),
                }
            }

            print!("config: {}", proj_dirs.config_dir().to_str().unwrap());

            let config_file_path = proj_dirs.config_dir().join("config.toml");

            // read prevoius congfiguration

            // process new data

            // geo cords by input address
            let city = "Lviv";

            let api_key = "93f07d077b39852d58b8db64cc351e0a";

            let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}&units=metric", city, api_key);

            let resp = reqwest::blocking::get(url).unwrap().text().unwrap();


            let locations: Vec<GeoInfo> = serde_json::from_str(&resp).unwrap();

            // write configuration 

            let app_cfg = AppConfiguration{
                location: Some(Location {
                    lat: 345.567,
                    lon: -45.4 
                }),
                preferred_provider: Some(String::from("openweather")),
                providers: Providers {
                    openweather: Some(ProviderConfiguration {
                            api_key: String::from("assdgdsdfsdfgsf")
                        }),
                    weatherapi: Some(ProviderConfiguration {
                        api_key: String::from("assdgdasdsdfsdfgsf")
                    }),
                },
            };

            let toml_string = toml::to_string(&app_cfg).unwrap();

            // Write the TOML string to a file
            let mut file = std::fs::File::create(config_file_path).unwrap();
            file.write_all(toml_string.as_bytes()).unwrap();
        }
        Some(Commands::Get(cfg)) => {
            println!("Address: {:?}", cfg.location);

            let parsed_time = parse_time(cfg.time.as_str()).unwrap();

            println!("Time: {:?}", parsed_time);
        }
        _ => {
            println!("Hello, world!");
        }
    }
}