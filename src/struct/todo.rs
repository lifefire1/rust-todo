use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    username: String,
    title: String,
    completed: bool,
    content: String,
}

impl Todo {
    fn new(username: String, title: String, completed: bool, content: String) -> Todo {
        Self {
            username,
            title,
            completed,
            content,
        }
    }
}