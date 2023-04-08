use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
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
    // #[arg(short, long)]
    provider: String,
}

#[derive(Args)]
struct GetArgs {
    address: String,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Configure(cfg)) => {
            println!("Provider: {:?}", cfg.provider)
        }
        Some(Commands::Get(cfg)) => {
            println!("Address: {:?}", cfg.address)
        }
        _ => {
            println!("Hello, world!")
        }
    }
}