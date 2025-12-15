use crate::utils::DATE_FORMAT;
use chrono;
pub struct Todo {
    pub date: chrono::DateTime<chrono::Local>,
    pub content: String,
    pub file_path: String,
}

impl Todo {
    pub fn new(date: Option<&str>, content: &str, file_path: &str) -> Self {
        if let Some(date) = date {
            Todo {
                date: chrono::DateTime::parse_from_str(date, DATE_FORMAT)
                    .unwrap()
                    .into(),
                content: String::from(content),
                file_path: String::from(String::from(file_path)),
            }
        } else {
            Todo {
                date: chrono::offset::Local::now(),
                content: String::from(content),
                file_path: String::from(String::from(file_path)),
            }
        }
    }
}
