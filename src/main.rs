pub mod board;
pub mod engine;
pub mod game;
pub mod logic;

use crate::game::GameState;
use crate::logic::Logic;
use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::RwLock;
use rocket::{get, launch, post, routes, State};
use serde_json::Value;
use std::env;

#[get("/")]
async fn handle_index(logic: &State<RwLock<Logic>>) -> Json<Value> {
    Json(logic.read().await.info())
}

#[post("/start", format = "json", data = "<start_req>")]
async fn handle_start(logic: &State<RwLock<Logic>>, start_req: Json<GameState>) -> Status {
    logic.write().await.start(&start_req);

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
async fn handle_move(logic: &State<RwLock<Logic>>, move_req: Json<GameState>) -> Json<Value> {
    let response = logic.write().await.get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
async fn handle_end(logic: &State<RwLock<Logic>>, end_req: Json<GameState>) -> Status {
    logic.write().await.end(&end_req);

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

    info!("Starting Battlesnake Server...");

    let logic = Logic::new();

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "cogsandsquigs/github/ferrite");
            })
        }))
        .manage(RwLock::new(logic))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
