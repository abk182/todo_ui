use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub type Response = Vec<Todo>;

pub struct State {
    pub loading: bool,
    pub response: Option<Response>,
}

pub struct Request {
    pub state: Arc<Mutex<State>>,
}

impl Request {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(State {
            loading: true,
            response: None,
        }));

        let background_state = Arc::clone(&state);

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let response = fetch_todos().await.unwrap();
                *background_state.lock().unwrap() = State {
                    loading: false,
                    response: Some(response),
                };
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // В нативной версии используем обычный thread
            std::thread::spawn(move || {
                // Создаем одноразовый runtime для выполнения async задачи
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime");

                // Выполняем HTTP запрос
                let result = rt.block_on(fetch_todos()).unwrap();
                *background_state.lock().unwrap() = State {
                    loading: false,
                    response: Some(result),
                };
            });
        }

        Request { state }
    }
}

async fn fetch_todos() -> Result<Response, String> {
    let url = "https://jsonplaceholder.typicode.com/todos?_limit=5";
    reqwest::get(url)
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON error: {}", e))
}

