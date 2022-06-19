use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files;

use askama::Template;

use dotenv::dotenv;

use std::env;

use chrono;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Homepage server running..");

    dotenv().ok();

    let listen_host = match env::var("LISTEN_HOST") {
        Ok(var) => var,
        Err(_) => String::from("0.0.0.0")
    };

    let listen_port = match env::var("LISTEN_PORT") {
        Ok(var) => var.parse().expect("Port has to be a number!"),
        Err(_) => 3000
    };

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/dist", "./dist"))
    })
    .bind((listen_host, listen_port))?
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
        description: format!("rendered at: {}", chrono::Local::now()),
    };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}
