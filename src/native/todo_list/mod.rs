mod todo;

use crate::utils::{DATE_FORMAT, FORMAT, TODOS};
use eframe::egui;
use std::{fs, io::Read, io::Write};
use todo::Todo;

pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        let entities = fs::read_dir(TODOS).unwrap();
        let mut list: Vec<Todo> = vec![];
        for entity in entities {
            let file_name = entity
                .unwrap()
                .file_name()
                .into_string()
                .unwrap_or_else(|err| panic!("can't convert file name into string. {:?}", err));
            let file_path = &format!("{0}/{1}", TODOS, file_name);
            let mut content = String::new();
            fs::File::open(file_path)
                .unwrap()
                .read_to_string(&mut content)
                .unwrap();
            let (date, content) = content.split_once("\n").unwrap();
            list.push(Todo::new(Some(date), content, file_path));
        }

        let todo_list = TodoList { list };

        todo_list
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut should_add_todo = false;
        let mut indices_to_update = Vec::new();
        let mut indices_to_delete = Vec::new();

        ui.horizontal(|ui| {
            for (index, todo) in self.list.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    let date_str = format!("Date: \n{}", todo.date.format(DATE_FORMAT));

                    let label = egui::Label::new(date_str);

                    let text_edit = egui::TextEdit::multiline(&mut todo.content)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true);

                    ui.add(label);
                    ui.add(text_edit);
                    if ui.add(egui::Button::new("Save")).clicked() {
                        indices_to_update.push(index);
                    }
                    if ui.add(egui::Button::new("Delete")).clicked() {
                        indices_to_delete.push(index);
                    }
                });
                ui.vertical(|ui| should_add_todo = ui.add(egui::Button::new("Add")).clicked());
                ui.add_space(32.0);
            }
        });

        if (should_add_todo) {
            let file_name = self.list.len().to_string();
            self.list.push(Todo::new(
                None,
                "A",
                &format!("{0}/{1}.{2}", TODOS, file_name, FORMAT),
            ));
        }

        for &index in indices_to_update.iter().rev() {
            if index < self.list.len() {
                let todo = &self.list[index];
                let content = String::from(format!(
                    "{}\n{}",
                    todo.date.format(DATE_FORMAT),
                    todo.content
                ));
                let mut file = fs::File::create(&todo.file_path).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }
        }

        for &index in indices_to_delete.iter().rev() {
            if index < self.list.len() {
                let todo = &self.list[index];
                fs::remove_file(&todo.file_path).unwrap();
                self.list.remove(index);
            }
        }
    }
}
