use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long)]
    output: PathBuf,

    inputs: Vec<PathBuf>,

    #[arg(short = 'e', long)]
    exclude: Vec<PathBuf>,

}

fn main() {
    let cli = Cli::parse();
}

