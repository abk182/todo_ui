use eframe::egui;
pub struct App {}

impl App {
    pub fn new(_creation_context: &eframe::CreationContext) -> Self {
        App {}
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
        });
    }
}
