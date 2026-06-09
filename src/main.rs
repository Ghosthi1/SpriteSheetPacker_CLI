//! CLI tool that takes png sprites and combines them into a spritesheet png with a JSON metadata file

use clap::Parser;
use std::{collections::HashSet, path::PathBuf};
use image::DynamicImage;
use serde::Serialize;
use rayon::prelude::*;

#[derive(Parser)]
struct Cli {
    /// Where the spritesheet and JSON metadata will go and what it will be named
    #[arg(short = 'o',long)]
    output: PathBuf,
    /// A png file or a directory holding png files
    inputs: Vec<PathBuf>,
    /// These directories and or files will be ignored
    #[arg(short = 'e',long)]
    exclude: Vec<PathBuf>,
    /// Sets the width of the spritesheet, Defaults to size of max sprite
    #[arg(short = 'w',long)]
    width: Option<u32>,
}

/// Images that are loaded into memory but not yet packed
struct LoadedImage {
    path: PathBuf,
    image: DynamicImage,
}

/// A loaded image with its position in the atlas assigned
struct PackedImage {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    image: DynamicImage,
}

/// Atlas metadata for a single sprite written to the JSON output
#[derive(Serialize)]
struct Sprite {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

/// collects the images from the directory, loads them all into memory, sorts them with biggest at the top, packs them all together,
/// combines them all and saves
fn main() {
    let cli = Cli::parse();
    let mut all_pngs = vec![];
    let mut exclude = cli.exclude;
    exclude.push(cli.output.with_extension("png")); // to prevent output PNG from being picked up on repeat runs
    let exclude_set: HashSet<PathBuf> = exclude.into_iter().collect();

    for input in &cli.inputs {
        all_pngs.extend(collect_pngs(input, &exclude_set));
    }
    let mut loaded = load_image(all_pngs);
    if loaded.is_empty(){
        eprintln!("No PNG files found in the provided inputs.") ;
        return
    }
    // Sorts the tallest first gives the shelf algorithm the best chance to minimize wasted space
    loaded.sort_by(|a, b| b.image.height().cmp(&a.image.height()));

    let auto_width = loaded.iter().map(|img| img.image.width()).max().unwrap_or(512);
    let width = cli.width.unwrap_or(auto_width);
    let sprites = pack_sprites(loaded, width);
    let (canvas, metadata) = composite(sprites, width);

    save_output(&cli.output, &canvas, &metadata);
}

/// Recursively looks through directories for png files ,ignores and files or directories in exclude,
/// return all valid png's in a vec
fn collect_pngs(path: &PathBuf, exclude: &HashSet<PathBuf>) -> Vec<PathBuf> {
    if exclude.contains(path) {
        vec![]
    }
    else if path.extension() == Some(std::ffi::OsStr::new("png")) {
        return vec![path.clone()];
    }
    else if path.is_dir() {
        path.read_dir().unwrap()
            .flat_map(|entry| collect_pngs(&entry.unwrap().path(), exclude))
            .collect()
    }
    else {
        vec![]
    }
}

/// Loads images into memory in parallel, skips files with an error and returns a warning
fn load_image(images: Vec<PathBuf>) ->  Vec<LoadedImage> {
    images.into_par_iter().filter_map(|path| {
        match image::open(&path) {
            Ok(temp_image) => Some(LoadedImage { path, image: temp_image }),
            Err(e) => {eprintln!("Failed to load {}: {}", path.display(), e); None}
        }
    }).collect()
}

/// Assigns x/y positions to each sprite using a shelf-packing algorithm.
/// Expects sprites pre-sorted tallest-first for best packing efficiency.
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

/// Combines all the png files into one spritesheet
fn composite (sprites: Vec<PackedImage>, max_width: u32) -> (image::RgbaImage, Vec<Sprite>) {
    let max_height = sprites.iter().map(|s| s.y + s.height).max().unwrap_or(0);
    let mut canvas = image::RgbaImage::new(max_width, max_height);
    let mut metadata = Vec::new();

    for sprite in sprites {
        image::imageops::overlay(&mut canvas, &sprite.image, sprite.x as i64, sprite.y as i64);
        metadata.push(Sprite { name: sprite.name, x: sprite.x, y: sprite.y, width: sprite.width, height: sprite.height });
    }
    (canvas,metadata)
}

/// outputs the sprite sheet and the json file
fn save_output(output: &PathBuf, canvas: &image::RgbaImage, metadata: &Vec<Sprite>) {
    let image_path = output.with_extension("png");
    let json_path = output.with_extension("json");
    if let Some(parent) = image_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    canvas.save(&image_path).expect("Failed to save atlas");
    let json = serde_json::to_string_pretty(&metadata).expect("Failed to serialize");
    std::fs::write(&json_path, json).expect("Failed to write JSON");
}

