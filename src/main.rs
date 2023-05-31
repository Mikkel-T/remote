mod volume;

use askama::Template;
use autopilot::key::Code;
use autopilot::key::KeyCode::{self, LeftArrow, RightArrow, Space};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
/// The data that a WebSocket message should have to be understood by the `handle_message` function
struct WsMessage {
    action: String,
    data: Option<f32>,
}

#[derive(Template)]
#[template(path = "remote.html")]
/// The data that should be passed when rendering the remote template
struct RemoteTemplate {
    volume: u32,
}

#[derive(Clone, Copy)]
enum MessageAction {
    Pause,
    Right,
    Left,
    Mute,
    Unmute,
    Vol(f32),
}

#[tokio::main]
async fn main() {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    // GET / -> remote.html
    let index = warp::path::end().map(|| RemoteTemplate {
        volume: (volume::get().unwrap() * 100.) as u32,
    });

    // GET /ws -> Initiate websocket connection
    let realtime = warp::path("ws")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| ws_connected(socket, users)));

    let routes = index.or(realtime);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn tap(key: KeyCode) {
    autopilot::key::tap(&Code(key), &[], 0, 0);
}

/// The WebSocket loop. Runs the `handle_message` function each time something is sent from the client.
async fn ws_connected(ws: WebSocket, users: Users) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {e}");
                break;
            }
        };

        let action = parse_message(msg);
        handle_action(action);
        if let Some(MessageAction::Vol(vol)) = action {
            for (&uid, tx) in users.read().await.iter() {
                if my_id != uid {
                    tx.send(Message::text(vol.to_string())).unwrap_or_else(|e| {
                        eprintln!("websocket send error: {}", e);
                    });
                }
            }
        }
    }

    users.write().await.remove(&my_id);
}

/// Function to handle the WebSocket messages and act on the messages.
fn parse_message(msg: Message) -> Option<MessageAction> {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return None;
    };

    let p: WsMessage = serde_json::from_str(msg).unwrap();

    match p.action.as_str() {
        "pause" => Some(MessageAction::Pause),
        "right" => Some(MessageAction::Right),
        "left" => Some(MessageAction::Left),
        "mute" => Some(MessageAction::Mute),
        "unmute" => Some(MessageAction::Unmute),
        "vol" => Some(MessageAction::Vol(p.data.unwrap())),
        _ => None,
    }
}

fn handle_action(action: Option<MessageAction>) {
    if let Some(a) = action {
        match a {
            MessageAction::Pause => tap(Space),
            MessageAction::Right => tap(RightArrow),
            MessageAction::Left => tap(LeftArrow),
            MessageAction::Mute => volume::mute().unwrap(),
            MessageAction::Unmute => volume::unmute().unwrap(),
            MessageAction::Vol(vol) => volume::set(vol / 100.).unwrap(),
        }
    }
}
