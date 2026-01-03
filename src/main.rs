use eframe::egui;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct RegexRule {
	pattern: String,
	replacement: String,
}

#[derive(Default, Serialize, Deserialize)]
struct RegexReplacerApp {
	input_text: String,
	output_text: String,
	rules: Vec<RegexRule>,
	errors: Vec<String>,
	width:String
}
impl RegexReplacerApp {
	fn add_button(&mut self, ui: &mut egui::Ui){
		if ui.button("+").clicked(){
			self.rules.push(RegexRule::default());
		}
	}
	fn input_output(&mut self, ui:&mut egui::Ui){
		ui.vertical(|ui|{
			ui.label("Input");
			ui.text_edit_multiline(&mut self.input_text);
		});
		ui.vertical(|ui|{
			ui.label("Out");
			ui.text_edit_multiline(&mut self.output_text);
		});
	}
}
fn regex_stack(ui: &mut egui::Ui, rule:&mut RegexRule){
	ui.text_edit_singleline(&mut rule.pattern);
	ui.text_edit_singleline(&mut rule.replacement);
}
impl eframe::App for RegexReplacerApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_visuals(egui::Visuals::dark());
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui|{
				let available = ui.available_width();
				self.width = format!("{}", available);
				ui.text_edit_singleline(&mut self.width);
				ui.vertical(|ui|{
					ui.label("Errors");
					ui.text_edit_multiline(&mut self.errors.join("\n"));
				});
				if available > 450.0 {
					ui.horizontal(|ui| {self.input_output(ui);});
				} else {
					ui.vertical(|ui| {self.input_output(ui);});
				}
				ui.vertical(|ui|{
					ui.heading("Regexes");
					if ui.button("Apply").clicked(){
						let mut text = self.input_text.clone();
						let mut errored = false;
						for rule in &self.rules{
							match Regex::new(&rule.pattern){
								Ok(re) => {
									text = re
										.replace_all(&text, rule.replacement.replace("\\n", "\n"))
										.to_string();
								}
								Err(e) => {
									self.errors.push(format!("Error: {e}"));
									errored = true;
									break;
								}
							}
						}
						if !errored{self.output_text = text;}
					}
					self.add_button(ui);
					for rule in &mut self.rules{
						if available > 450.0 {
							ui.horizontal(|ui|{regex_stack(ui, rule);});
						} else {
							ui.vertical(|ui|{regex_stack(ui, rule);});
							ui.separator();
						}
					}
					self.add_button(ui);
				});
			});
		});
	}
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Regex Replacer",
        options,
        Box::new(|cc| {
            let mut app = RegexReplacerApp::default();
            if let Some(storage) = cc.storage {
                if let Some(stored) =
					eframe::get_value::<RegexReplacerApp>(storage, eframe::APP_KEY) {
						app = stored;
					}
            }
            Ok(Box::new(app))
        }),
    )
}
