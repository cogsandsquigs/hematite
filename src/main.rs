pub mod configuration;
pub mod engine;
pub mod game;
pub mod server;

use crate::game::GameState;
use crate::server::Server;
use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::RwLock;
use rocket::{get, launch, post, routes, State};
use serde_json::Value;
use std::env;

#[get("/")]
async fn handle_index(server: &State<RwLock<Server>>) -> Json<Value> {
    Json(server.read().await.info())
}

#[post("/start", format = "json", data = "<start_req>")]
async fn handle_start(server: &State<RwLock<Server>>, start_req: Json<GameState>) -> Status {
    server.write().await.start(&start_req);

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
async fn handle_move(server: &State<RwLock<Server>>, move_req: Json<GameState>) -> Json<Value> {
    let response = server.write().await.get_move(&move_req);

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
async fn handle_end(server: &State<RwLock<Server>>, end_req: Json<GameState>) -> Status {
    server.write().await.end(&end_req);

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Snake Server...");

    let server = Server::new();

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "cogsandsquigs/github/hematite");
            })
        }))
        .manage(RwLock::new(server))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
