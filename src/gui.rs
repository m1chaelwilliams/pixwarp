use egui::*;
use eframe::*;
use rfd::FileDialog;
use std::path::Path;

use crate::converter::{export_file, validate_file_exists, validate_new_file, validate_path_exists};

fn get_file() -> Option<String> {
	if let Some(file) = FileDialog::new()
		.add_filter("png", &["png", "PNG"])
		.add_filter("jpg", &["jpg", "jpeg", "JPG", "JPEG"])
		.add_filter("bmp", &["bmp", "BMP"])
		.add_filter("tiff", &["tiff", "TIFF"])
		.add_filter("ico", &["ico", "ICO"])
		.add_filter("avif", &["avif", "AVIF"])
		.add_filter("webp", &["webp", "WEBP"])
		.pick_file()
	{
		return Some(file.to_string_lossy().into_owned().to_string());
	}
	None
}

fn get_dir() -> Option<String> {
	if let Some(dir) = FileDialog::new()
		.pick_folder()
	{
		return Some(dir.to_string_lossy().into_owned().to_string());
	}
	None
}

pub enum Message {
	ClearFields,
	Convert,
	LocateFilepath,
	LocateOutDir,
	ClearErrors,
}

#[derive (Default)]
pub struct ConversionApp {
	init_filepath: String,
	out_dir: String,
	out_name: String,
	fields_valid: bool,
	fields_checked: bool,
	complaint: String,
}

impl ConversionApp {
	pub fn new(_cc: &CreationContext<'_>) -> Self {
		Self::default()
	}
}

impl App for ConversionApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		CentralPanel::default().show(ctx, |ui| {
			ui.heading("Image Converter");
			ui.separator();
			ui.add_space(15.0);

			Grid::new("Input Fields")
				.num_columns(3)
				.striped(true)
				.spacing([15.0, 5.0])
				.show(ui, |ui| {
					self.input_fields(ui)
			});

			ui.add_space(15.0);
			ui.separator();
			ui.add_space(5.0);

			ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
				self.action_buttons(ui);
			});			
		});
	}
}

impl ConversionApp {
	fn on_message(&mut self, message: Message) {
		match message {
			Message::ClearFields => {
				self.init_filepath.clear();
				self.out_dir.clear();
				self.out_name.clear();
				self.fields_checked = false;
			},
			Message::Convert => {
				if let Some(init_filepath) = validate_file_exists(self.init_filepath.clone()) {
					let out_dir = Path::new(&self.out_dir);

					if let Some(_) = validate_path_exists(self.out_dir.clone()) {
						if let Some(_) = validate_new_file(self.out_name.clone()) {
							
							let out_filepath = out_dir.join(Path::new(&self.out_name));

							match export_file(&init_filepath, &out_filepath.to_string_lossy().to_string()) {
								Ok(_) => {
									self.fields_valid = true;
									self.fields_checked = false;
									return;
								},
								Err(e) => {
									println!("{}", e);
								}
							};

						} else {
							self.complaint = "Output filename is not valid.".to_string();
						}
					} else {
						self.complaint = "Output filepath does not exist".to_string();
					}
				} else {
					self.complaint = "Initial filepath does not exist".to_string();
				}
				self.fields_checked = true;
				self.fields_valid = false;
			},
			Message::LocateFilepath => {
				if let Some(filepath) = get_file() {
					self.init_filepath = filepath;
					self.fields_checked = false;
				}
			},
			Message::LocateOutDir => {
				if let Some(dir) = get_dir() {
					self.out_dir = dir;
					self.fields_checked = false;
				}
			},
			Message::ClearErrors => {
				self.fields_checked = false;
				self.fields_valid = false;
				self.complaint.clear();
			}
		}
	}

	fn input_fields(&mut self, ui: &mut Ui) {
		ui.label("File to Convert:");
		if ui.add(
			egui::TextEdit::singleline(&mut self.init_filepath)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("path/to/example.jpg")	
		).changed() {
			self.on_message(Message::ClearErrors);
		};
		if ui.button("Locate").clicked() {
			self.on_message(Message::LocateFilepath);
		};
		ui.end_row();

		ui.label("Output Directory:");
		if ui.add(
			egui::TextEdit::singleline(&mut self.out_dir)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("output/directory/")
		).changed() {
			self.on_message(Message::ClearErrors);
		};
		if ui.button("Locate").clicked() {
			self.on_message(Message::LocateOutDir);
		};
		ui.end_row();

		ui.label("Output Name:");
		if ui.add(
			egui::TextEdit::singleline(&mut self.out_name)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("Example.jpg")	
		).changed() {
			self.on_message(Message::ClearErrors);
		};
		ui.end_row();
	}

	fn action_buttons(&mut self, ui: &mut Ui) {
		if ui.button("Clear").clicked() {
			self.on_message(Message::ClearFields);
		};
		if ui.button("Convert").clicked() {
			self.on_message(Message::Convert);
		};

		if self.fields_checked && !self.fields_valid {
			ui.colored_label(Color32::from_rgb(255, 0, 0), &self.complaint);
		}
	}
}