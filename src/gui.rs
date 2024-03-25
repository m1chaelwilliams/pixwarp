use egui::*;
use eframe::*;
use rfd::FileDialog;

fn get_file() -> Option<String> {
	if let Some(file) = FileDialog::new()
		.add_filter("png", &["png", "PNG"])
		.add_filter("jpg", &["jpg", "JPG"])
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
}

#[derive (Default)]
pub struct ConversionApp {
	init_filepath: String,
	out_dir: String,
	out_name: String,
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

			ui.separator();
			ui.add_space(15.0);

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
			},
			Message::Convert => (),
			Message::LocateFilepath => {
				if let Some(filepath) = get_file() {
					self.init_filepath = filepath;
				}
			},
			Message::LocateOutDir => {
				if let Some(dir) = get_dir() {
					self.out_dir = dir;
				}
			},
		}
	}

	fn input_fields(&mut self, ui: &mut Ui) {
		ui.label("File to Convert:");
		ui.add(
			egui::TextEdit::singleline(&mut self.init_filepath)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("path/to/example.jpg")	
		);
		if ui.button("Locate").clicked() {
			self.on_message(Message::LocateFilepath);
		};
		ui.end_row();

		ui.label("Output Directory:");
		ui.add(
			egui::TextEdit::singleline(&mut self.out_dir)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("output/directory/")	
		);
		if ui.button("Locate").clicked() {
			self.on_message(Message::LocateOutDir);
		};
		ui.end_row();

		ui.label("Output Name:");
		ui.add(
			egui::TextEdit::singleline(&mut self.out_name)
				.min_size(Vec2::new(150.0, 1.0))
				.hint_text("Example.jpg")	
		);
		ui.end_row();
	}

	fn action_buttons(&mut self, ui: &mut Ui) {
		if ui.button("Clear").clicked() {
			self.on_message(Message::ClearFields);
		};
		if ui.button("Convert").clicked() {
			self.on_message(Message::Convert);
		};
	}
}