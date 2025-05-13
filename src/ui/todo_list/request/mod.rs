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

                let mut state = background_state.lock().unwrap();
                state.response = Some(response);
                state.loading = false;
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
                let result = rt.block_on(fetch_todos_from_server());

                // Сохраняем результат в общее состояние
                {
                    let mut state = background_state.lock().unwrap();
                    state.result = Some(result);
                    state.is_loading = false;
                }

                // Запрашиваем перерисовку UI
                ctx.request_repaint();
            });
        }

        Request { state }
    }
}

/// Асинхронная функция для загрузки задач с сервера
pub async fn fetch_todos() -> Result<Response, String> {
    // URL сервера для получения задач
    let url = "https://jsonplaceholder.typicode.com/todos?_limit=5";

    // Выполняем HTTP запрос
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    // Проверяем статус ответа
    if !response.status().is_success() {
        return Err(format!("Server returned error: {}", response.status()));
    }

    // Парсим JSON
    let response = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;

    Ok(response)
}
