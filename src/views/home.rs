use maud::{html, Markup};

use crate::models::todo_list::TodoList;

use super::base::base_view;

pub fn home_view(create_todo_url: String, todo_lists: Vec<TodoList>) -> Markup {
    base_view(html! {
        h1 { "Todo App" }
        button hx-get=(create_todo_url) hx-swap="outerHTML #app" class="btn btn-primary" {
            "New Todo Todo"
        }
        select class="form-select" aria-label="Default select example" {
            @for todo_list in todo_lists {
                option value={(todo_list.name)} {
                    (todo_list.name)
                }
            }
        }

    })
}
