use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files;

use askama::Template;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/dist", "./dist"))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

struct BaseTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "index.html.j2")]
struct IndexTemplate {
    base: BaseTemplate,
    description: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let tmpl = IndexTemplate {
        base: BaseTemplate {
            title: String::from("Hello world")
        },
        description: String::from("This site is running.")
    };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}
