use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};
use std::io;

#[get("/")]
async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Hello World" }
            }
            body {
                header { "Hello World" }
                h1 { "Hello World!" }
                p { "Welcome to the Hello World app!" }
                p { "This is a simple example of a Rust web app using Actix Web and Maud." }
                footer { "© 2024 Rust Web App" }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}