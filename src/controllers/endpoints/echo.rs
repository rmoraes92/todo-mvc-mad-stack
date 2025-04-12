use actix_web::{HttpResponse, Responder, get, web::Path};
use maud::{Markup, html};

// HTMLX

#[get("/hello/{world}", name = "echo")]
async fn echo(path: Path<String>) -> impl Responder {
    let world = path.into_inner();
    let content: Markup = html! {
        h1 { "Hello, " (world) "!" }
    };
    HttpResponse::Ok().body(content.into_string())
}
