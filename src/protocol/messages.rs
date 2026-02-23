use crate::social::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const TOPIC_PREFIX: &str = "gnunet/social";

pub fn topic(path: &str) -> String {
    format!("{}/{}", TOPIC_PREFIX, path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Auth(AuthRequest),
    CreateUser(CreateUserRequest),
    UpdateUser(UpdateUserRequest),
    CreatePost(CreatePostRequest),
    GetFeed(GetFeedRequest),
    GetPost(GetPostRequest),
    LikePost(LikePostRequest),
    CreateRoom(CreateRoomRequest),
    GetRooms(GetRoomsRequest),
    JoinRoom(JoinRoomRequest),
    LeaveRoom(LeaveRoomRequest),
    SendRoomMessage(SendRoomMessageRequest),
    GetRoomMessages(GetRoomMessagesRequest),
    RequestFriend(RequestFriendRequest),
    AcceptFriend(AcceptFriendRequest),
    GetFriends(GetFriendsRequest),
    SendPrivateMessage(SendPrivateMessageRequest),
    GetPrivateMessages(GetPrivateMessagesRequest),
    GetUser(GetUserRequest),
    SearchUsers(SearchUsersRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub peer_id: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub media_hashes: Vec<String>,
    pub reply_to: Option<Uuid>,
    pub repost_of: Option<Uuid>,
    pub visibility: PostVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeedRequest {
    pub peer_id: String,
    pub limit: Option<u32>,
    pub before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostRequest {
    pub post_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikePostRequest {
    pub post_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_group: bool,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomsRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRoomRequest {
    pub room_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRoomMessageRequest {
    pub room_id: Uuid,
    pub content: String,
    pub media_hashes: Vec<String>,
    pub reply_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomMessagesRequest {
    pub room_id: Uuid,
    pub limit: Option<u32>,
    pub before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFriendRequest {
    pub peer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptFriendRequest {
    pub peer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFriendsRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPrivateMessageRequest {
    pub recipient_id: String,
    pub content: String,
    pub media_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPrivateMessagesRequest {
    pub peer_id: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub peer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersRequest {
    pub query: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Auth(AuthResponse),
    User(UserResponse),
    Post(PostResponse),
    Feed(FeedResponse),
    Room(RoomResponse),
    RoomMessage(RoomMessageResponse),
    Friend(FriendResponse),
    PrivateMessage(PrivateMessageResponse),
    Error(ErrorResponse),
    Event(EventMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub peer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub post: Option<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedResponse {
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub room: Option<ChatRoom>,
    pub rooms: Option<Vec<ChatRoom>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMessageResponse {
    pub message: Option<ChatMessage>,
    pub messages: Option<Vec<ChatMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendResponse {
    pub friendship: Option<Friendship>,
    pub friends: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMessageResponse {
    pub message: Option<PrivateMessage>,
    pub messages: Option<Vec<PrivateMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum EventMessage {
    NewPost {
        post: Post,
    },
    NewRoomMessage {
        room_id: Uuid,
        message: ChatMessage,
    },
    NewPrivateMessage {
        message: PrivateMessage,
    },
    FriendRequest {
        from: String,
        friendship: Friendship,
    },
    FriendAccepted {
        peer_id: String,
    },
    UserOnline {
        peer_id: String,
    },
    UserOffline {
        peer_id: String,
    },
}

impl ErrorResponse {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
