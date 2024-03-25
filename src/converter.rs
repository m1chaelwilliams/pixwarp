use std::path::Path;
use colored::*;
use image::{self, DynamicImage};

pub fn supports_format(extension: &str) -> bool {
    match extension {
        "jpg" => true,
        "png" => true,
        "ico" => true,
        "bmp" => true,
        "tiff" => true,
        "avif" => true,
        "webp" => true,
        _ => false
    }
}

pub fn validate_file_exists(response: String) -> Option<String> {
	let path = Path::new(&response);
        
	if path.exists() && path.is_file() {
		if let Some(extension) = path.extension().to_owned() {
			let extension_str = extension.to_string_lossy();

			if supports_format(&extension_str.into_owned().as_str()) {
				return Some(response);
			} else {
				println!("{}", "File type not supported.".red().bold());
			}
		}
	}
	println!("{}", "Please enter a valid filepath.".red().bold());
	return None
}

pub fn validate_path_exists(response: String) -> Option<String> {
	let path = Path::new(&response);

	if path.exists() {
		return Some(response);
	}
	println!("{}", "Please enter a valid filepath.".red().bold());
	None
}

// validates file name and extension for file that does *not* exist. File
// must not be an empty string and must be a supported type.
pub fn validate_new_file(response: String) -> Option<String> {
	if response.len() > 0 {
		let path = Path::new(&response);

		if let Some(extension) = path.extension().to_owned() {
			let extension_str = extension.to_string_lossy();
		
			if supports_format(&extension_str.into_owned().as_str()) {
				return Some(response);
			} else {
				println!("{}", "File type not supported.".red().bold());
				return None;
			}
		} else {
			println!("{}", "Please enter a valid filename".red().bold());
			return None;
		}
	}
	println!("{}", "Name must not be blank.".red().bold());
	None
}

pub fn export_file(init_filepath: &String, out_filepath: &String) -> image::ImageResult<DynamicImage> {
	let init_path = Path::new(init_filepath);
	let out_path = Path::new(out_filepath);
	
	let img = image::io::Reader::open(init_path)?
		.decode()?;

	match img.save(out_path) {
		Ok(_) => Ok(img),
		Err(e) => Err(e)
	}
}