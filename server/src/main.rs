use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use tokio::sync::mpsc;

use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use std::collections::HashMap;

use crate::data::ROOMS;
use crate::data::TASKS;
use crate::message::Message;
use crate::room::Room;

mod data;
mod game;
mod message;
mod room;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        .fallback_service(get(show_lobbies_handler))
        .route("/create/:roomid", get(create_lobby_handler))
        .route("/join", get(join_lobby_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = "127.0.0.1:3000";
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn show_lobbies_handler() -> impl IntoResponse {
    let data = ROOMS.read().await;
    let available_rooms: HashMap<String, u8> = data
        .iter()
        .map(|(roomid, game)| (roomid.clone(), game.player_count()))
        .collect();
    Json::from(available_rooms)
}

async fn create_lobby_handler(
    axum::extract::Path(roomid): axum::extract::Path<String>,
) -> impl IntoResponse {
    if roomid.is_empty() || roomid.len() > 32 {
        return (StatusCode::BAD_REQUEST, "roomid is empty or too long");
    }

    let mut room_data = ROOMS.write().await;
    match room_data.get(&roomid) {
        Some(_) => (StatusCode::BAD_REQUEST, "room already exists"),
        None => {
            let mut room = Room::new();
            let (tx, rx) = room.init_game();
            room_data.insert(roomid.clone(), room);

            tracing::info!("created room \"{}\"", &roomid);

            let mut tasks = TASKS.lock().await;
            tasks.push((tx, tokio::spawn(game_task(roomid, rx))));

            (StatusCode::CREATED, "room created")
        }
    }
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
async fn join_lobby_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    todo!()
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, player_name: String) {
    todo!()
}

async fn game_task(roomid: String, mut rx: mpsc::Receiver<Message>) {
    loop {
        let mut rooms = ROOMS.write().await;
        let game = rooms.get_mut(&roomid).unwrap();
        match rx.try_recv() {
            Ok(msg) => match msg {
                Message::Join(player_name) => {
                    if let None = game.p1 {
                        tracing::info!(
                            "player \"{}\" logged into room \"{}\" as player 1",
                            &player_name,
                            &roomid
                        );
                        game.p1 = Some(player_name);
                    } else if let None = game.p2 {
                        tracing::info!(
                            "player \"{}\" logged into room \"{}\" as player 2",
                            &player_name,
                            &roomid
                        );
                        game.p2 = Some(player_name);
                    } else {
                        tracing::info!(
                            "player \"{}\" logged into room \"{}\" but it was all filled up!",
                            &player_name,
                            &roomid
                        );
                    }
                }
                Message::Leave(player_name) => {
                    tracing::info!("player \"{}\" left room \"{}\"", &player_name, &roomid);

                    tracing::info!("shutting down room \"{}\"", &roomid);
                    return;
                }
                Message::EndGame => {
                    tracing::info!("shutting down room \"{}\"", &roomid);
                    return;
                }
            },
            Err(mpsc::error::TryRecvError::Empty) => {
                continue;
            }
            Err(_) => {
                tracing::error!("all senders disconnected for roomid: \"{}\"", &roomid);

                tracing::info!("shutting down room \"{}\"", &roomid);
                return;
            }
        }

        if game.player_count() == 2 {
            break;
        }
    }
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
/// stolen from: https://github.com/joelparkerhenderson/demo-rust-axum/blob/main/README.md
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    tracing::info!("signal shutdown");

    tracing::info!("shutting down rooms");
    let mut tasks = TASKS.lock().await;
    for (tx, task) in tasks.iter_mut() {
        tx.send(Message::EndGame)
            .await
            .expect("failed to issue shutdown signal");
        task.await.expect("task failed to join");
    }
}
