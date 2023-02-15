use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "ewrer", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    New,
    Del,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
