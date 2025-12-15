use eframe::egui;
use todo_list::TodoList;

mod todo_list;

pub struct App {
    // TODO: add enum or trait
    widgets: Vec<TodoList>,
}

impl App {
    pub fn new(_creation_context: &eframe::CreationContext) -> Self {
        App {
            widgets: vec![TodoList::new()],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for widget in &mut self.widgets {
                widget.show(ui);
            }
        });
    }
}
