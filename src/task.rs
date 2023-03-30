use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoForm {
    pub description: String,
}
