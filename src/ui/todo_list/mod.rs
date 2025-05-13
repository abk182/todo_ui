use eframe::egui;
use request::Request;
use todo::Todo;

mod request;
mod todo;

pub struct TodoList {
    list: Vec<Todo>,
    request: Option<Request>,
}

impl TodoList {
    pub fn new() -> Self {
        let mut todo_list = TodoList {
            list: vec![],
            request: None,
        };

        todo_list.load_list();

        todo_list
    }

    fn load_list(&mut self) {
        self.request = Some(Request::new());
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.check_request_state();

        ui.horizontal(|ui| {
            for todo in &mut self.list {
                todo.show(ui);
                ui.add_space(32.0);
            }
        });
    }

    fn check_request_state(&mut self) {
        if let Some(req) = &self.request {
            let mut state = req.state.lock().unwrap();

            if (state.loading == false) {
                if let Some(response) = state.response.take() {
                    for todo in response {
                        self.list.push(Todo::new(String::from(todo.title)));
                    };
                }
            }
        }
    }
}
