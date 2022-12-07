#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;

use actix_web::{get, post, HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use models::{User, NewUser};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    autor: String,
}

#[derive(Debug, Deserialize)]
struct Loginuser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

fn estabilish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to: {}", database_url))
}

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("ZK Proofs"),
            link: String::from("https://google.com"),
            autor: String::from("Railgun")
        },
        Post {
            title: String::from("test2"),
            link: String::from("https://example2.com"),
            autor: String::from("me2")       
        }
    ];

    data.insert("title", "Ts");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/signup")]
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Succesfuly saved user: {}", data.username))
}

#[get("/signup")]
async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/login")]
async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/submission")]
async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submission");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/submission")]
async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("new sub: {} -> {}", data.title, data.link))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .service(index)
            .service(signup)
            .service(login)
            .service(process_signup)
            .service(submission)
            .service(process_submission)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}