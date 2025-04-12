use actix_web::{get, post, web::{self, Data, Form}, HttpRequest, HttpResponse, Responder};
use maud::{Markup, html};

use crate::{controllers::{dao::todo_list::create_todo_list, database::DatabaseConnectionPool}, models::{forms::todolist_create::NewTodoList, todo_list::TodoList}};

// HTMLX

#[get("/todo_list/create", name = "todolist-create")]
async fn todolist_create(
    req: HttpRequest,
) -> impl Responder {

    let create_todo_url = match req.url_for::<Vec<String>, String>("todolist-create", vec![]) {
        Ok(url) => url.to_string(),
        Err(e) => {
            eprintln!("Error generating URL: {}", e);
            String::from("#") // Fallback URL
        }
    };

    let content: Markup = html! {
        h2 { "New Todo" }
        form id="todolist"
        hx-post=(create_todo_url)
        hx-swap="outerHTML settle:3s"
        hx-target="#app" {
            input type="text" id="name" name="name" placeholder="Todo List Name" class="form-control";
            input type="submit" value="Create" class="btn btn-success";
        }
    };
    HttpResponse::Ok().body(content.into_string())
}

#[post("/todo_list/create", name = "todolist-create")]
async fn todolist_create_post(
    req: HttpRequest,
    form: Form<NewTodoList>,
    db_pool: Data<DatabaseConnectionPool>,
) -> impl Responder {

    let new_todolist: NewTodoList = form.into_inner();

    let todolist: TodoList = TodoList {
        id: 0,
        name: new_todolist.name.clone(),
    };

    let _ = web::block(move ||
        create_todo_list(
            &mut db_pool.get().unwrap(),
            todolist,
        ).unwrap()
    );

    let create_todo_url = match req.url_for::<Vec<String>, String>("todolist-create", vec![]) {
        Ok(url) => url.to_string(),
        Err(e) => {
            eprintln!("Error generating URL: {}", e);
            String::from("#") // Fallback URL
        }
    };

    let content: Markup = html! {
        h2 { "New Todo" }
        form id="todolist"
        hx-post=(create_todo_url)
        hx-swap="outerHTML settle:3s"
        hx-target="#app" {
            input type="text" id="name" name="name" placeholder="Todo List Name" class="form-control";
            input type="submit" value="Create" class="btn btn-success";
        }
    };
    HttpResponse::Ok().body(content.into_string())
}
