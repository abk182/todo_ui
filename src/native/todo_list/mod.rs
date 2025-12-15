mod todo;

use eframe::egui;
use std::{fs, io::Read};

use todo::Todo;

pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        let folder = "todos";
        let entities = fs::read_dir(folder).unwrap();
        let mut list: Vec<Todo> = vec![];
        for entity in entities {
            let file_name = entity
                .unwrap()
                .file_name()
                .into_string()
                .unwrap_or_else(|err| panic!("can't convert file name into string. {:?}", err));
            let mut content = String::new();
            fs::File::open(&format!("{0}/{1}", folder, file_name))
                .unwrap()
                .read_to_string(&mut content)
                .unwrap();
            let (date, content) = content.split_once("\n").unwrap();
            list.push(Todo::new(Some(date), content));
        }

        let todo_list = TodoList { list };

        todo_list
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for todo in &mut self.list {
                todo.show(ui);
                ui.add_space(32.0);
            }
        });
    }
}
