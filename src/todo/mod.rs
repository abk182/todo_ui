use crate::constants::DATE_FORMAT;
use chrono;
pub struct Todo {
    pub date: chrono::DateTime<chrono::Local>,
    pub content: String,
}

impl Todo {
    pub fn new(date: Option<&str>, content: &str) -> Self {
        let content = String::from(content);

        if let Some(date) = date {
            Todo {
                date: chrono::DateTime::parse_from_str(date, DATE_FORMAT)
                    .unwrap()
                    .into(),
                content,
            }
        } else {
            Todo {
                date: chrono::offset::Local::now(),
                content,
            }
        }
    }
}
