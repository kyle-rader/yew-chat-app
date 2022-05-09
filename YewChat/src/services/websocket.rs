use futures::{channel::mpsc::Sender, SinkExit, StreamExit};
use reqwasm::websocket::{futures::WebSocket, Message};

use wasm_bindgen_futures::spawn_local;

pub struct WebSocketService {
    pub tx: Send<String>,
}

impl WebSocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8080").expect("Failed to open web socket!");

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channels::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("from websocket: {}", data);
                    },
                    Ok(Message::Bytes(data)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log::debug!("from websocket: {}", data);
                        }
                    },
                    Err(e) => {
                        log::error!("ws: {:?}", e);
                    }
                }
            }
            log::debug!("WebSocket closed");
        });

        Self {
            tx: in_tx
        }
    }
}
