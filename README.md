# PixWarp Image Converter
Convert your images easily!

![Screenshot](screenshot.png "screenshot of the CLI")
![GUI Screenshot](guiscreenshot.png "Screenshot of the GUI (WIP)")

## Status
- CLI: Working!
- GUI: WIP (50%)

## Supported Formats:
- JPG
- PNG
- ICO
- BMP
- GIF
- WEBP
- AVIF
- TIFF

## Installation
Clone the repository and run:
```rust
cargo build --release
```
Find the resulting executable in `/target/release`.

## Dependencies
- [image](https://github.com/image-rs/image)
- [colored](https://github.com/colored-rs/colored)

## License
This software does not have its own license. It abides and follows the guidelines of its dependencies.

## TODO
I plan on adding an optional GUI. There will be compilation options for both headless and not.

## Author
Michael Williams