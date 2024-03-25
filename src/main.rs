mod prompt;
mod gui;

use std::io;
use std::path::Path;
use colored::*;

// personal modules
use prompt::*;
use gui::*;
use eframe::*;

use image;

fn supports_format(extension: &str) -> bool {
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

fn run_headless() -> io::Result<()> {
    println!("--- Image Converter ---");

    println!("{} Enter an filepath (including filename & extension):", ">".green());
    let init_filepath_raw = prompt(|response| {
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
    });

    let init_path = Path::new(&init_filepath_raw);

    println!("{} Enter directory for exported file:", ">".green());
    let out_dir_raw = prompt(|response| {
        let path = Path::new(&response);

        if path.exists() {
            return Some(response);
        }
        println!("{}", "Please enter a valid filepath.".red().bold());
        None
    });

    let out_dir = Path::new(&out_dir_raw);

    println!("{} Enter name (with extension) of exported file:", ">".green());
    let out_file_raw = prompt(|response| {
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
    });

    let out_file = Path::new(&out_file_raw);

    let img = image::io::Reader::open(init_path)?
        .decode().unwrap();

    println!("{}", "Converting...".yellow());
    match img.save(out_dir.join(out_file).as_path()) {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    };
    println!("{} {}", "Converted! output in:".green(), out_dir.to_string_lossy().cyan().bold());
    Ok(())
}

fn run_gui() -> io::Result<()> {
    match run_native(
        "Image Converter", 
        NativeOptions::default(), 
    Box::new(|cc| {
        Box::new(ConversionApp::new(cc))
    })) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(io::Error::new(io::ErrorKind::Other, "Failed to run GUI"))
        }
    }
}

fn main() -> io::Result<()> {
    #[cfg(feature = "headless")]
    if cfg!(feature = "headless") {
        run_headless()?;
    }

    #[cfg(feature = "gui")]
    if cfg!(feature = "gui") {
        run_gui()?;
    }

    Ok(())
}