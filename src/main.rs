use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long)]
    output: PathBuf,

}

fn main() {
    let cli = Cli::parse();
}

