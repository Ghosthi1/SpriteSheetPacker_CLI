# Sprite Sheet Packer

A CLI tool that packs PNG sprites into a single atlas image and a JSON metadata file describing each sprite's position and size.

## Build

```bash
cargo build --release
```

The binary will be at `target/release/sprite-packer`.

## Usage

```
sprite-packer -o <output> [OPTIONS] <inputs>...
```

| Flag                  | Description                                                               |
|-----------------------|---------------------------------------------------------------------------|
| `-o, --output <path>` | Output base path — produces `<path>.png` and `<path>.json`                |
| `<inputs>...`         | PNG files or directories to pack (directories are searched recursively)   |
| `-e, --exclude <path>`| File or directory to skip (repeatable)                                    |
| `-w, --width <px>`    | Atlas width in pixels (defaults to the widest sprite)                     |

## Example

```bash
sprite-packer -o output/atlas sprites/ hero.png enemy.png
```

This packs all PNGs found in `sprites/`, plus `hero.png` and `enemy.png`, into:

- `output/atlas.png` — the combined sprite sheet
- `output/atlas.json` — metadata for each sprite

## JSON output format

```json
[
  { "name": "hero", "x": 0, "y": 0, "width": 64, "height": 64 },
  { "name": "enemy", "x": 64, "y": 0, "width": 32, "height": 48 }
]
```

`name` is the file stem (filename without extension). `x`/`y` are the top-left pixel coordinates in the atlas.

## Notes

- The output PNG is automatically excluded from input collection, so re-running the same command is safe.
- Images that fail to load are skipped with a warning printed to stderr.
- Sprites are packed using a shelf algorithm, sorted tallest-first to minimize wasted space.