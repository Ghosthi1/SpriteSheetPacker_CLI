# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Rust CLI tool (edition 2024) for packing sprite sheets. Core functionality complete.

## Teaching Mode

The user is here to learn Rust. **Do not edit source files.** Instead:
- Guide the user to write the code themselves
- Explain concepts before suggesting what to type
- Ask questions to check understanding
- Point to relevant docs or stdlib types when helpful
- Review code the user has written and give feedback

## Commands

```bash
cargo build          # compile (debug)
cargo build --release  # compile (optimized)
cargo run            # build and run
cargo test           # run all tests
cargo clippy         # lint
cargo fmt            # format
```

## Design Decisions

- **Input:** accepts a mix of files and directories; directories recurse into subdirectories unless `--exclude` is passed
- **Output:** packed PNG atlas + JSON metadata file (sprite name, x, y, width, height)
- **Argument parsing:** `clap` v4 with the derive feature
- **CLI shape:**
  ```
  sprite-packer -o output/atlas sprites/ hero.png enemy.png
  ```
  Output base path derives both files: `output/atlas.png` + `output/atlas.json`
- **Atlas width:** auto-calculated from widest sprite; override with `-w <px>`
- **Self-exclusion:** output PNG is automatically excluded from input collection to prevent re-packing on repeated runs

## Dependencies

- `clap = { version = "4", features = ["derive"] }` — CLI argument parsing
- `image = "0.25"` — PNG loading, pixel manipulation, saving
- `serde = { version = "1", features = ["derive"] }` — serialization
- `serde_json = "1"` — JSON output

## Architecture

Single binary crate (`src/main.rs`). Modules:
- CLI argument definition (clap derive struct) ✅
- Input collection — `collect_pngs(path, exclude)` recurses directories, filters `.png` ✅
- Image loading — `load_image(images)` opens each PNG, stores `LoadedImage { path, image }`, prints error and skips on failure ✅
- Bin-packing — `pack_sprites(images, max_width)` shelf algorithm, returns `Vec<PackedImage>` ✅
- Image compositing — `composite(sprites, max_width)` draws onto `RgbaImage` canvas, returns `(canvas, Vec<Sprite>)` ✅
- Output — saves atlas PNG + JSON metadata, auto-creates output directory ✅