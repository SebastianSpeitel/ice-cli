mod api;

use api::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if the ICE API is available
    Check,
    /// Generate a prompt to be used in starship
    Prompt {
        #[arg(
            short,
            long,
            default_value = "{train_type} {vzn} -> {final_station} ({speed} km/h)"
        )]
        format: String,
    },
}

fn main() {
    env_logger::init();
    let api = Api::new();
    let args = Cli::parse();

    if let Commands::Check = args.command {
        if !api.available() {
            eprintln!("ICE API is not available.");
            std::process::exit(1);
        }
        eprintln!("ICE API is available.");
        std::process::exit(0);
    }

    if let Commands::Prompt { format } = args.command {
        let status = match api.status() {
            Ok(status) => status,
            Err(e) => {
                eprintln!("Error: {:#?}", e);
                std::process::exit(1);
            }
        };
        let trip = match api.trip() {
            Ok(trip) => trip.trip,
            Err(e) => {
                eprintln!("Error: {:#?}", e);
                std::process::exit(1);
            }
        };

        let output = format
            .replace("{vzn}", &trip.vzn)
            .replace("{tzn}", &status.tzn)
            .replace("{final_station}", &trip.stop_info.final_station_name)
            .replace("{train_type}", &status.train_type.to_string())
            .replace("{speed}", &status.speed.to_string());
        println!("{}", output);
        std::process::exit(0);
    }
}
