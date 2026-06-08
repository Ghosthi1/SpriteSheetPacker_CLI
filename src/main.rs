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

    let mut all_pngs = vec![];
    for input in &cli.inputs {
        all_pngs.extend(collect_pngs(input, &cli.exclude));
    }
    println!("{:#?}", all_pngs);
}

fn collect_pngs(path: &PathBuf, exclude: &Vec<PathBuf>) -> Vec<PathBuf> {
    if path.extension() == Some(std::ffi::OsStr::new("png")) {
        return vec![path.clone()];
    }
    if exclude.contains(&path) {
        return vec![];
    }
    if path.is_dir() {
        let mut results = vec![];
        for entry in path.read_dir().unwrap() {
            let entry_path = entry.unwrap().path();
            results.extend(collect_pngs(&entry_path, exclude));
        }
        return results;
    }
    vec![]
}
