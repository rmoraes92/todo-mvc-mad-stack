use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    web::{self, Data},
};
use maud::Markup;

use crate::controllers::database::DatabaseConnectionPool;
use crate::{controllers::dao::todo_list::get_todo_list, views::home::home_view};

#[get("/", name = "home")]
async fn home(
    req: HttpRequest,
    db_pool: Data<DatabaseConnectionPool>,
) -> impl Responder {
    let todo_list =
        match web::block(move || get_todo_list(&mut db_pool.get().unwrap()).unwrap()).await {
            Ok(todo_list) => todo_list,
            Err(e) => {
                eprintln!("Error fetching todo list: {}", e); // TODO remove me later
                vec![]
            }
        };

    let create_todo_url = match req.url_for::<Vec<String>, String>("todolist-create", vec![]) {
        Ok(url) => url.to_string(),
        Err(e) => {
            eprintln!("Error generating URL: {}", e);
            String::from("#") // Fallback URL
        }
    };

    let content: Markup = home_view(create_todo_url, todo_list);

    HttpResponse::Ok().body(content.into_string())
}
