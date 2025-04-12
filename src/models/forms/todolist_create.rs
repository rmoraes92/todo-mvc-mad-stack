use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewTodoList {
    pub name: String,
}