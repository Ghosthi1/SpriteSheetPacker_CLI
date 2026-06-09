use std::collections::HashSet;
use clap::Parser;
use std::path::PathBuf;
use image::{DynamicImage};
use serde::Serialize;
use rayon::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'o', long)]
    output: PathBuf,
    inputs: Vec<PathBuf>,
    #[arg(short = 'e', long)]
    exclude: Vec<PathBuf>,
    #[arg(short = 'w' ,long)]
    width: Option<u32>,
}

struct LoadedImage {
    path: PathBuf,
    image: DynamicImage,
}

struct PackedImage {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    image: DynamicImage,
}

#[derive(Serialize)]
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
    let mut exclude = cli.exclude;
    exclude.push(cli.output.with_extension("png"));
    let exclude_set: HashSet<PathBuf> = exclude.into_iter().collect();

    for input in &cli.inputs {
        all_pngs.extend(collect_pngs(input, &exclude_set));
    }
    let mut loaded = load_image(all_pngs);
    if loaded.is_empty(){eprintln!("No PNG files found in the provided inputs.") ;return}
    loaded.sort_by(|a, b| b.image.height().cmp(&a.image.height()));

    let auto_width = loaded.iter().map(|img| img.image.width()).max().unwrap_or(512);
    let width = cli.width.unwrap_or(auto_width);
    let sprites = pack_sprites(loaded, width);
    let (canvas, metadata) = composite(sprites, width);

    let image_path = cli.output.with_extension("png");
    let json_path = cli.output.with_extension("json");
    if let Some(parent) = image_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    canvas.save(&image_path).expect("Failed to save atlas");
    let json = serde_json::to_string_pretty(&metadata).expect("Failed to serialize");
    std::fs::write(&json_path, json).expect("Failed to write JSON");
}

fn collect_pngs(path: &PathBuf, exclude: &HashSet<PathBuf>) -> Vec<PathBuf> {
    if exclude.contains(path) {
        return vec![];
    }
    if path.extension() == Some(std::ffi::OsStr::new("png")) {
        return vec![path.clone()];
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
    images.into_par_iter().filter_map(|path| {
        match image::open(&path) {
            Ok(temp_image) => Some(LoadedImage { path, image: temp_image }),
            Err(e) => {eprintln!("Failed to load {}: {}", path.display(), e); None}
        }
    }).collect()
}

fn pack_sprites(images: Vec<LoadedImage>, max_width: u32) -> Vec<PackedImage>{
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

        result.push(PackedImage { name, x, y, width: img_width, height: img_height , image: image.image});
        x += img_width;
        row_height = row_height.max(img_height);
    }
    result
}

fn composite (sprites: Vec<PackedImage>, max_width: u32 ) -> (image::RgbaImage, Vec<Sprite>) {
    let max_height = sprites.iter().map(|s| s.y + s.height).max().unwrap_or(0);
    let mut canvas = image::RgbaImage::new(max_width, max_height);
    let mut metadata = Vec::new();

    for sprite in sprites {
        image::imageops::overlay(&mut canvas, &sprite.image, sprite.x as i64, sprite.y as i64);
        metadata.push(Sprite { name: sprite.name, x: sprite.x, y: sprite.y, width: sprite.width, height: sprite.height });
    }
    (canvas,metadata)
}

