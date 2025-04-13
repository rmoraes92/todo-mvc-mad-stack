use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post,
    web::{self, Data, Form},
};
use maud::{Markup, html};

use crate::{
    controllers::{dao::todo_list::{create_todo_list, research_todo_list}, database::DatabaseConnectionPool},
    models::{forms::todolist_create::NewTodoList, todo_list::{TodoListCreate, TodoList}},
    views::home::home_view,
};

// HTMLX

#[get("/todo_list/create", name = "todolist-create")]
async fn todolist_create(req: HttpRequest) -> impl Responder {
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

    let todolist = TodoListCreate {
        name: new_todolist.name.clone(),
    };

    let mut conn = db_pool.get().unwrap();

    let x = web::block(move || create_todo_list(&mut conn, todolist).unwrap());

    dbg!(&x.await.as_ref().unwrap());

    let create_todo_url = match req.url_for::<Vec<String>, String>("todolist-create", vec![]) {
        Ok(url) => url.to_string(),
        Err(e) => {
            eprintln!("Error generating URL: {}", e);
            String::from("#") // Fallback URL
        }
    };

    let todo_list: Vec<TodoList> =
        match web::block(move || research_todo_list(&mut db_pool.get().unwrap()).unwrap()).await {
            Ok(todo_list) => todo_list,
            Err(e) => {
                eprintln!("Error fetching todo list: {}", e); // TODO remove me later
                vec![]
            }
        };

    dbg!(&todo_list);

    let content: Markup = home_view(create_todo_url, todo_list);

    HttpResponse::Ok().body(content.into_string())
}
