use crate::utils::DATE_FORMAT;
use chrono;
use eframe::egui;
pub struct Todo {
    date: chrono::DateTime<chrono::Local>,
    content: String,
}

impl Todo {
    pub fn new(date: Option<&str>, content: &str) -> Self {
        if let Some(date) = date {
            Todo {
                date: chrono::DateTime::parse_from_str(date, DATE_FORMAT)
                    .unwrap()
                    .into(),
                content: String::from(content),
            }
        } else {
            Todo {
                date: chrono::offset::Local::now(),
                content: String::from(content),
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let date_str = format!("Date: \n{}", self.date.format(DATE_FORMAT));

            let label = egui::Label::new(date_str);

            let text_edit = egui::TextEdit::multiline(&mut self.content)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true);

            let button = egui::Button::new("Save");

            ui.add(label);
            ui.add(text_edit);
            ui.add(button);
        });
    }
}
