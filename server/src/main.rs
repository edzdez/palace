use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};

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

use crate::data::DATA;
use crate::room::Room;

mod data;
mod game;
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
        .route("/join", post(join_lobby_handler))
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
    let data = DATA.read().await;
    let available_rooms: HashMap<String, u8>  = data
        .iter()
        .map(|(roomid, game)| {
            (roomid.clone(), game.player_count())
        })
        .collect();
    Json::from(available_rooms)
}

async fn create_lobby_handler(
    axum::extract::Path(roomid): axum::extract::Path<String>
) -> impl IntoResponse {
    if roomid.is_empty() || roomid.len() > 32 {
        return (StatusCode::BAD_REQUEST, "roomid too long")
    }

    let mut data = DATA.write().await;
    match data.get(&roomid) {
        Some(_) => {
            (StatusCode::BAD_REQUEST, "roomid already exists")
        }
        None => {
            let room = Room::new();
            data.insert(roomid, room);
            (StatusCode::CREATED, "room created")
        }
    }
}

async fn join_lobby_handler() -> impl IntoResponse {
    todo!()
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
/// stolen from: https://github.com/joelparkerhenderson/demo-rust-axum/blob/main/README.md
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
