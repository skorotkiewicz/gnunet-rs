use crate::gnunet::PeerIdentity;
use crate::social::ChatMessage;
use futures::channel::mpsc::{channel, Receiver, Sender};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ChannelId = u64;
pub type PortId = String;

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: ChannelId,
    pub peer: PeerIdentity,
    pub port: PortId,
}

#[derive(Debug, Clone)]
pub struct CadetMessage {
    pub channel: ChannelId,
    pub data: Vec<u8>,
}

pub struct CadetService {
    channels: std::collections::HashMap<ChannelId, Channel>,
    ports: std::collections::HashMap<PortId, Sender<CadetMessage>>,
    next_channel_id: ChannelId,
}

impl Default for CadetService {
    fn default() -> Self {
        Self::new()
    }
}

impl CadetService {
    pub fn new() -> Self {
        Self {
            channels: std::collections::HashMap::new(),
            ports: std::collections::HashMap::new(),
            next_channel_id: 1,
        }
    }

    pub fn open_port(&mut self, port: &str) -> Receiver<CadetMessage> {
        let (tx, rx) = channel(1024);
        self.ports.insert(port.to_string(), tx);
        rx
    }

    pub fn close_port(&mut self, port: &str) {
        self.ports.remove(port);
    }

    pub fn create_channel(&mut self, peer: PeerIdentity, port: &str) -> Channel {
        let id = self.next_channel_id;
        self.next_channel_id += 1;

        let channel = Channel {
            id,
            peer,
            port: port.to_string(),
        };

        self.channels.insert(id, channel.clone());
        channel
    }

    pub fn destroy_channel(&mut self, channel_id: ChannelId) {
        self.channels.remove(&channel_id);
    }

    pub fn send(&mut self, channel_id: ChannelId, data: Vec<u8>) -> bool {
        let port = self.channels.get(&channel_id).map(|c| c.port.clone());
        if let Some(port) = port {
            if let Some(sender) = self.ports.get_mut(&port) {
                let _ = sender.try_send(CadetMessage {
                    channel: channel_id,
                    data,
                });
                return true;
            }
        }
        false
    }

    pub fn broadcast(&mut self, port: &str, data: Vec<u8>) {
        let channel_ids: Vec<ChannelId> = self
            .channels
            .values()
            .filter(|c| c.port == port)
            .map(|c| c.id)
            .collect();

        for id in channel_ids {
            self.send(id, data.clone());
        }
    }
}

pub const SOCIAL_PORT: &str = "social";
pub const CHAT_PORT: &str = "chat";
pub const FILESHARE_PORT: &str = "fileshare";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cadet_type", rename_all = "snake_case")]
pub enum SocialCadetMessage {
    Post {
        post_id: Uuid,
        author: String,
        content: String,
    },
    Chat {
        room_id: Uuid,
        message: ChatMessage,
    },
    FriendRequest {
        from: String,
    },
    FriendAccept {
        to: String,
    },
    PrivateMessage {
        message: crate::social::PrivateMessage,
    },
}
