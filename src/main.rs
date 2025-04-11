use eframe::egui;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator",
        eframe::NativeOptions::default(),
        Box::new(|creation_context| Ok(Box::new(App::new(creation_context)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");

    let canvas = document
        .create_element("canvas")
        .expect("Failed to create canvas");
    body.append_child(canvas.as_ref())
        .expect("Failed to append canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let runner = eframe::WebRunner::new();

    runner.start(
        canvas,
        eframe::WebOptions::default(),
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

struct App {}

impl App {
    fn new(_creation_context: &eframe::CreationContext) -> Self {
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
