mod prompt;
#[cfg(feature = "gui")]
mod gui;
mod converter;

#[cfg(windows)]
use std::{ffi::c_ulong, io};

// personal modules
#[cfg(feature = "headless")]
use std::path::Path;

#[cfg(feature = "headless")]
use colored::*;

#[cfg(feature = "headless")]
use prompt::*;

#[cfg(feature = "headless")]
use converter::*;

use std::io;

#[cfg(feature = "gui")]
use gui::*;

#[cfg(feature = "gui")]
use eframe::*;

#[cfg(feature = "headless")]
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

    println!("{}", "Converting...".yellow());

    let _img_result = export_file(&init_filepath_raw, &out_filepath_raw.to_string());

    println!("{} {}", "Converted! output in:".green(), out_dir.to_string_lossy().cyan().bold());
    Ok(())
}

#[cfg(feature = "gui")]
fn run_gui() -> io::Result<()> {
    use egui::{IconData, ViewportBuilder, Vec2};

    // empty icon (because I don't have one haha)
    let custom_icon = IconData {
        rgba: vec![0, 0, 0, 0],
        width: 1,
        height: 1
    };

    match run_native(
        "Image Converter", 
        NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size(Vec2::new(400.0, 180.0)).with_icon(custom_icon),
            ..Default::default()
        }, 
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

#[cfg(windows)]
extern "system" {
    fn FreeConsole() -> c_ulong;
}

// free the console that automatically opens on windows
#[cfg(windows)]
fn free_console() {
    unsafe {
        FreeConsole();
    }
}

#[cfg(not(windows))]
fn free_console() {
    
}

fn main() -> io::Result<()> {

    #[cfg(feature = "headless")]
    if cfg!(feature = "headless") {
        return run_headless();
    }

    #[cfg(feature = "gui")]
    if cfg!(feature = "gui") {

        // freeing the terminal
        free_console();
        
        return run_gui();
    }

    Ok(())
}