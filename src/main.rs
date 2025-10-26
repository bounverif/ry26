use clap::{Parser, Subcommand};
use ry26::{DataPoint, add, from_json, generate_random_data_point, to_json};
use std::process;

/// A simple CLI for the ry26 library
#[derive(Parser)]
#[command(name = "ry26")]
#[command(about = "A command line interface for the ry26 library", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two numbers together
    Add {
        /// First number
        left: u64,
        /// Second number
        right: u64,
    },
    /// Generate a random data point and output as JSON
    Generate,
    /// Convert data point to JSON
    ToJson {
        /// ID of the data point
        #[arg(long)]
        id: u64,
        /// Value of the data point
        #[arg(long)]
        value: f64,
        /// Timestamp of the data point (ISO 8601 format)
        #[arg(long)]
        timestamp: String,
    },
    /// Parse JSON string and display data point
    FromJson {
        /// JSON string to parse
        json: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { left, right } => {
            let result = add(left, right);
            println!("{}", result);
        }
        Commands::Generate => {
            let data = generate_random_data_point();
            match to_json(&data) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::ToJson {
            id,
            value,
            timestamp,
        } => {
            let data = DataPoint {
                id,
                value,
                timestamp,
            };
            match to_json(&data) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::FromJson { json } => match from_json(&json) {
            Ok(data) => {
                println!("ID: {}", data.id);
                println!("Value: {}", data.value);
                println!("Timestamp: {}", data.timestamp);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
    }
}
