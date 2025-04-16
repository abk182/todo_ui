mod todo_list;

use eframe::egui;
use todo_list::TodoList;

pub struct App {
    todo_list: TodoList,
}

impl App {
    pub fn new(_creation_context: &eframe::CreationContext) -> Self {
        App {
            todo_list: TodoList::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label(r#"  
            Это вымышленный калькулятов.  
            Чтобы воспользоваться калькулятором вам необходимо воспользоваться воображением.            Представте себе любой интерфейс, наберите в нем математическое выражение и нажмите '='.  
            Увидели результат, Да? - Поздравляю, ваш калькулятор работает хорошо.  
            "#);  

            self.todo_list.show(ui);
        });
    }
}
