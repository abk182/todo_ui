use eframe::egui;

pub struct TodoList {
    list: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            list: vec![String::from("first"), String::from("second")],
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        for el in &self.list {
            ui.add(egui::Label::new(el));
        }

        for el in &mut self.list {
            let text_edit = egui::TextEdit::multiline(el)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true);

            ui.add(text_edit);
        }
    }
}
