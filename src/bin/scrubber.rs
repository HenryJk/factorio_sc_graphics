use std::error::Error;
use std::fs::read_dir;
use std::path::Path;
use image::{image_dimensions, RgbImage};
use std::env::args;

fn scrub_path(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        for entry in read_dir(path)? {
            let subpath = entry?.path();
            scrub_path(&subpath)?;
        }
    } else {
        println!("Scrubbing {}", path.to_string_lossy());
        let dimension = image_dimensions(&path)?;
        let empty_image = RgbImage::new(dimension.0, dimension.1);
        empty_image.save(&path)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = args().collect();
    let working_dir = String::from(".");
    let output_dir = if argv.len() > 1 {&argv[1]} else {&working_dir};
    scrub_path(Path::new(output_dir))?;
    Ok(())
}
