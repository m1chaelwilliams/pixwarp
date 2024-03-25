mod prompt;
mod gui;
mod converter;

use std::io;
use std::path::Path;
use colored::*;

// personal modules
use prompt::*;
use gui::*;
use converter::*;
use eframe::*;

fn run_headless() -> io::Result<()> {
    println!("--- Image Converter ---");

    println!("{} Enter an filepath (including filename & extension):", ">".green());
    let init_filepath_raw = prompt(validate_file_exists);

    let _init_path = Path::new(&init_filepath_raw);

    println!("{} Enter directory for exported file:", ">".green());
    let out_dir_raw = prompt(validate_path_exists);

    let out_dir = Path::new(&out_dir_raw);

    println!("{} Enter name (with extension) of exported file:", ">".green());
    let out_file_raw = prompt(validate_new_file);

    let out_file = Path::new(&out_file_raw);

    let out_filepath = out_dir.join(out_file);
    let out_filepath_raw = out_filepath.to_string_lossy();

    let _img_result = export_file(&init_filepath_raw, &out_filepath_raw.to_string());

    // let img = image::io::Reader::open(init_path)?
    //     .decode().unwrap();

    println!("{}", "Converting...".yellow());
    // match img.save(out_dir.join(out_file).as_path()) {
    //     Ok(_) => (),
    //     Err(e) => println!("{}", e)
    // };
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