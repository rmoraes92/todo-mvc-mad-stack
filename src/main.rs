use actix_web::{App, HttpServer, web::Data};

use todo_mash_v2::controllers::{
    database::build_db_conn_pool,
    endpoints::{todolist_create::{todolist_create, todolist_create_post}, echo::echo, home::home},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = build_db_conn_pool()
        .await
        .expect("Failed to create database pool");

    // Start Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(echo)
            .service(home)
            .service(todolist_create)
            .service(todolist_create_post)
        //.route("/", web::get().to(hello))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
