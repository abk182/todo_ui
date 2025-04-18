use eframe::egui;
use todo::Todo;

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

    pub fn load_list(&mut self) {
        self.list.push(Todo::new(String::from("first")));
        self.list.push(Todo::new(String::from("second")));
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
