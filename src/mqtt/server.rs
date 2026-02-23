use super::handler::MessageHandler;
use crate::protocol::*;
use crate::social::SocialStore;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct MqttServer {
    store: SocialStore,
    handler: Arc<MessageHandler>,
    event_tx: broadcast::Sender<ServerMessage>,
    connected_peers: Arc<RwLock<HashMap<String, String>>>,
}

impl MqttServer {
    pub fn new() -> Self {
        let store = SocialStore::new();
        let handler = Arc::new(MessageHandler::new(store.clone()));
        let (event_tx, _) = broadcast::channel(1024);

        Self {
            store,
            handler,
            event_tx,
            connected_peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_store(store: SocialStore) -> Self {
        let handler = Arc::new(MessageHandler::new(store.clone()));
        let (event_tx, _) = broadcast::channel(1024);

        Self {
            store,
            handler,
            event_tx,
            connected_peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<ServerMessage> {
        self.event_tx.subscribe()
    }

    pub fn process_message(&self, payload: &[u8]) -> Option<ServerMessage> {
        match serde_json::from_slice::<ClientMessage>(payload) {
            Ok(msg) => {
                let response = self.handler.handle(msg);
                if let ServerMessage::Auth(ref auth) = response {
                    if auth.success {
                        self.connected_peers
                            .write()
                            .insert(auth.peer_id.clone(), "connected".to_string());
                    }
                }
                Some(response)
            }
            Err(e) => {
                tracing::error!("Failed to parse message: {}", e);
                Some(ServerMessage::Error(ErrorResponse::new(
                    400,
                    "Invalid message format",
                )))
            }
        }
    }

    pub fn broadcast_event(&self, event: EventMessage) {
        let msg = ServerMessage::Event(event);
        let _ = self.event_tx.send(msg);
    }

    pub fn get_connected_peers(&self) -> Vec<String> {
        self.connected_peers.read().keys().cloned().collect()
    }

    pub fn get_store(&self) -> &SocialStore {
        &self.store
    }
}

pub fn topic_for_user(peer_id: &str) -> String {
    topic(&format!("user/{}", peer_id))
}

pub fn topic_for_feed(peer_id: &str) -> String {
    topic(&format!("feed/{}", peer_id))
}

pub fn topic_for_room(room_id: &str) -> String {
    topic(&format!("room/{}", room_id))
}

pub fn topic_for_events(peer_id: &str) -> String {
    topic(&format!("events/{}", peer_id))
}

pub fn topic_global_events() -> String {
    topic("events/global")
}
