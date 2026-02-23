use crate::gnunet::PeerIdentity;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub type PeerId = PeerIdentity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: PeerId,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_hash: Option<String>,
    pub gns_zone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, gns_zone: String) -> Self {
        let now = Utc::now();
        Self {
            id: PeerId::default(),
            username,
            display_name: String::new(),
            bio: None,
            avatar_hash: None,
            gns_zone,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: PeerId,
    pub content: String,
    pub media_hashes: Vec<String>,
    pub reply_to: Option<Uuid>,
    pub repost_of: Option<Uuid>,
    pub visibility: PostVisibility,
    pub created_at: DateTime<Utc>,
    pub likes: Vec<String>,
    pub reposts: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PostVisibility {
    Public,
    FollowersOnly,
    MutualsOnly,
    Private,
}

impl Post {
    pub fn new(author_id: PeerId, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author_id,
            content,
            media_hashes: Vec::new(),
            reply_to: None,
            repost_of: None,
            visibility: PostVisibility::Public,
            created_at: Utc::now(),
            likes: Vec::new(),
            reposts: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: PeerId,
    pub admins: Vec<PeerId>,
    pub members: Vec<PeerId>,
    pub is_group: bool,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

impl ChatRoom {
    pub fn new(name: String, owner_id: PeerId, is_group: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            owner_id: owner_id.clone(),
            admins: vec![owner_id.clone()],
            members: vec![owner_id],
            is_group,
            is_public: false,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub sender_id: PeerId,
    pub content: String,
    pub media_hashes: Vec<String>,
    pub reply_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(room_id: Uuid, sender_id: PeerId, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            room_id,
            sender_id,
            content,
            media_hashes: Vec::new(),
            reply_to: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friendship {
    pub requester_id: PeerId,
    pub addressee_id: PeerId,
    pub status: FriendshipStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Friendship {
    pub fn new(requester_id: PeerId, addressee_id: PeerId) -> Self {
        let now = Utc::now();
        Self {
            requester_id,
            addressee_id,
            status: FriendshipStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn accept(&mut self) {
        self.status = FriendshipStatus::Accepted;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMessage {
    pub id: Uuid,
    pub sender_id: PeerId,
    pub recipient_id: PeerId,
    pub content: String,
    pub media_hashes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl PrivateMessage {
    pub fn new(sender_id: PeerId, recipient_id: PeerId, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            content,
            media_hashes: Vec::new(),
            created_at: Utc::now(),
            read_at: None,
        }
    }
}

pub type UserStore = Arc<RwLock<HashMap<String, User>>>;
pub type PostStore = Arc<RwLock<HashMap<Uuid, Post>>>;
pub type RoomStore = Arc<RwLock<HashMap<Uuid, ChatRoom>>>;
pub type MessageStore = Arc<RwLock<HashMap<Uuid, ChatMessage>>>;
pub type FriendshipStore = Arc<RwLock<HashMap<String, Friendship>>>;
pub type PrivateMessageStore = Arc<RwLock<HashMap<Uuid, PrivateMessage>>>;

fn friendship_key(a: &PeerId, b: &PeerId) -> String {
    let mut ids = vec![a.as_str(), b.as_str()];
    ids.sort();
    ids.join(":")
}

#[derive(Debug, Clone)]
pub struct SocialStore {
    pub users: UserStore,
    pub posts: PostStore,
    pub rooms: RoomStore,
    pub messages: MessageStore,
    pub friendships: FriendshipStore,
    pub private_messages: PrivateMessageStore,
}

impl Default for SocialStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SocialStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            posts: Arc::new(RwLock::new(HashMap::new())),
            rooms: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
            friendships: Arc::new(RwLock::new(HashMap::new())),
            private_messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_user(&self, user: User) {
        self.users
            .write()
            .insert(user.id.as_str().to_string(), user);
    }

    pub fn get_user(&self, id: &str) -> Option<User> {
        self.users.read().get(id).cloned()
    }

    pub fn add_post(&self, post: Post) {
        self.posts.write().insert(post.id, post);
    }

    pub fn get_post(&self, id: Uuid) -> Option<Post> {
        self.posts.read().get(&id).cloned()
    }

    pub fn get_posts_by_author(&self, author_id: &PeerId) -> Vec<Post> {
        self.posts
            .read()
            .values()
            .filter(|p| p.author_id == *author_id)
            .cloned()
            .collect()
    }

    pub fn add_room(&self, room: ChatRoom) {
        self.rooms.write().insert(room.id, room);
    }

    pub fn get_room(&self, id: Uuid) -> Option<ChatRoom> {
        self.rooms.read().get(&id).cloned()
    }

    pub fn add_message(&self, msg: ChatMessage) {
        self.messages.write().insert(msg.id, msg);
    }

    pub fn get_room_messages(&self, room_id: Uuid) -> Vec<ChatMessage> {
        self.messages
            .read()
            .values()
            .filter(|m| m.room_id == room_id)
            .cloned()
            .collect()
    }

    pub fn request_friendship(&self, friendship: Friendship) {
        let key = friendship_key(&friendship.requester_id, &friendship.addressee_id);
        self.friendships.write().insert(key, friendship);
    }

    pub fn get_friendship(&self, a: &PeerId, b: &PeerId) -> Option<Friendship> {
        let key = friendship_key(a, b);
        self.friendships.read().get(&key).cloned()
    }

    pub fn accept_friendship(&self, a: &PeerId, b: &PeerId) -> bool {
        let key = friendship_key(a, b);
        if let Some(f) = self.friendships.write().get_mut(&key) {
            f.accept();
            true
        } else {
            false
        }
    }

    pub fn get_friends(&self, user_id: &PeerId) -> Vec<PeerId> {
        self.friendships
            .read()
            .values()
            .filter(|f| f.status == FriendshipStatus::Accepted)
            .filter(|f| f.requester_id == *user_id || f.addressee_id == *user_id)
            .map(|f| {
                if f.requester_id == *user_id {
                    f.addressee_id.clone()
                } else {
                    f.requester_id.clone()
                }
            })
            .collect()
    }

    pub fn add_private_message(&self, msg: PrivateMessage) {
        self.private_messages.write().insert(msg.id, msg);
    }

    pub fn get_private_messages(&self, user_id: &PeerId) -> Vec<PrivateMessage> {
        self.private_messages
            .read()
            .values()
            .filter(|m| m.sender_id == *user_id || m.recipient_id == *user_id)
            .cloned()
            .collect()
    }
}
