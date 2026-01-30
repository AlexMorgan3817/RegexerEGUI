use eframe::egui;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegexRule {
	pattern: String,
	replacement: String,
	case_sensitive: bool,
}
impl RegexRule {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            replacement: String::new(),
            case_sensitive: true,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct RegexReplacerApp {
	input_text: String,
	output_text: String,
	rules: Vec<RegexRule>,
	errors: Vec<String>,
	width:String
}

impl eframe::App for RegexReplacerApp {
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_visuals(egui::Visuals::dark());
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui|{
				let available = ui.available_width();
				self.width = format!("{}", available);
				// Width and errors debug
				if cfg!(debug_assertions){
					ui.text_edit_singleline(&mut self.width);
					ui.vertical(|ui|{
						ui.label("Errors");
						ui.text_edit_multiline(&mut self.errors.join("\n"));
					});
				}
				// Input / Output
				if available > 450.0
					{ui.horizontal(|ui| {self.input_output(ui)});}
				else
					{ui.vertical(|ui| {self.input_output(ui)});}
				// Regexes table
				ui.vertical(|ui|{
					self.regexes_table(ui, &available);
				});
			});
		});
	}
}

impl RegexReplacerApp {
	fn button_apply_regexes(&mut self){
		let mut text = self.input_text.clone();
		let mut errored = false;
		for idx in (0..self.rules.len()).rev(){
			let rule = &self.rules[idx];
			let p = &rule.pattern;
			let px = if rule.case_sensitive { p.clone() } else { format!("(?i){}", p)};
			match Regex::new(&px){
				Ok(pattern) => {
					text = pattern
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
		if !errored{
			self.output_text = text;
		}
	}

	fn regexes_table(&mut self, ui: &mut egui::Ui, available: &f32){
		ui.heading("Regexes");
		if ui.button("Apply").clicked(){
			self.button_apply_regexes();
		}
		self.button_add_regex(ui);
		// Table itself
		for idx in (0..(&self.rules).len()).rev(){
			ui.horizontal(|ui|{
				if *available > 450.0 {
					ui.horizontal(|ui|{
						self.regex_stack(ui, idx);
					});
				} else {
					ui.vertical(|ui|{
						self.regex_stack(ui, idx);
						ui.separator();
					});
				}
			});
		}
		self.button_add_regex(ui);
	}

	fn regex_stack(&mut self, ui: &mut egui::Ui, idx:usize){
		let l =  self.rules.len();
		let r = self.rules.get_mut(idx);
		match r {
			Some(rule) => {
				ui.text_edit_singleline(&mut rule.pattern);
				ui.text_edit_singleline(&mut rule.replacement);
				if ui.button("-").clicked(){
					self.rules.remove(idx);
					return;
				}
				ui.checkbox(&mut rule.case_sensitive, "Case Sensitive");
				ui.horizontal(|ui|{
					if ui.button("/\\").clicked(){
						if idx < l - 1{
							self.rules.swap(idx, idx + 1);
						}
					}
					if ui.button("\\/").clicked(){
						if idx > 0{
							self.rules.swap(idx, idx - 1);
						}
					}
				});
			},
			None => {
				ui.label(format!("Error: No rule found at index {}", idx));
			}
		}
	}
	fn button_add_regex(&mut self, ui: &mut egui::Ui){
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
