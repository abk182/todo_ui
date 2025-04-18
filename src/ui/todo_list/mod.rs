use eframe::egui;
use todo::Todo;

use crate::ui::utils::file::list_files;

mod todo;

pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        let mut todo_list = TodoList { list: vec![] };

        todo_list.load_list();

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

    fn load_list(&mut self) {
        let files = list_files(None);
        
        println!("{:?}", files);

        for text in vec![String::from("first todo"), String::from("second todo")] {
            self.list.push(Todo::new(String::from(text)));
        }
    }
}
