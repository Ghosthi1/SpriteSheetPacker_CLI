use std::char::MAX;
use std::cmp::max;
use clap::Parser;
use std::path::PathBuf;
use image::DynamicImage;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long)]
    output: PathBuf,

    inputs: Vec<PathBuf>,

    #[arg(short = 'e', long)]
    exclude: Vec<PathBuf>,

    #[arg(short = 'w' ,long, default_value_t = 512)]
    width: u32,
}

struct LoadedImage {
    path: PathBuf,
    image: DynamicImage,
}

struct Sprite {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let cli = Cli::parse();
    let mut all_pngs = vec![];

    for input in &cli.inputs {
        all_pngs.extend(collect_pngs(input, &cli.exclude));
    }
    let sprites = pack_sprites(load_image(all_pngs), cli.width);
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

fn load_image(images: Vec<PathBuf>) ->  Vec<LoadedImage> {
    let mut result = vec![];
    for path in images {
        match image::open(&path) {
            Ok(temp_image) => result.push(LoadedImage { path, image: temp_image }),
            Err(e) => eprintln!("Failed to load {}: {}", path.display(), e),
        }
    }
    result
}

fn pack_sprites(images: Vec<LoadedImage>, max_width: u32) -> Vec<Sprite> {
    let mut x = 0u32;
    let mut y = 0u32;
    let mut row_height = 0u32;
    let mut result = vec![];

    for image in images {
        let img_width = image.image.width();
        let img_height = image.image.height();
        let name = image.path.file_stem().unwrap().to_string_lossy().to_string();

        if x + img_width > max_width {
            x = 0;
            y += row_height;
            row_height = 0;
        }

        result.push(Sprite { name, x, y, width: img_width, height: img_height });
        x += img_width;
        row_height = row_height.max(img_height);
    }
    result
}
