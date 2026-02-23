use crate::gnunet::PeerIdentity;
use crate::protocol::*;
use crate::social::*;
use std::sync::Arc;

pub struct MessageHandler {
    store: SocialStore,
    current_peer: Arc<parking_lot::RwLock<Option<PeerIdentity>>>,
}

impl MessageHandler {
    pub fn new(store: SocialStore) -> Self {
        Self {
            store,
            current_peer: Arc::new(parking_lot::RwLock::new(None)),
        }
    }

    pub fn set_current_peer(&self, peer: PeerIdentity) {
        *self.current_peer.write() = Some(peer);
    }

    pub fn current_peer(&self) -> Option<PeerIdentity> {
        self.current_peer.read().clone()
    }

    pub fn handle(&self, msg: ClientMessage) -> ServerMessage {
        match msg {
            ClientMessage::Auth(req) => self.handle_auth(req),
            ClientMessage::CreateUser(req) => self.handle_create_user(req),
            ClientMessage::UpdateUser(req) => self.handle_update_user(req),
            ClientMessage::CreatePost(req) => self.handle_create_post(req),
            ClientMessage::GetFeed(req) => self.handle_get_feed(req),
            ClientMessage::GetPost(req) => self.handle_get_post(req),
            ClientMessage::LikePost(req) => self.handle_like_post(req),
            ClientMessage::CreateRoom(req) => self.handle_create_room(req),
            ClientMessage::GetRooms(req) => self.handle_get_rooms(req),
            ClientMessage::JoinRoom(req) => self.handle_join_room(req),
            ClientMessage::LeaveRoom(req) => self.handle_leave_room(req),
            ClientMessage::SendRoomMessage(req) => self.handle_send_room_message(req),
            ClientMessage::GetRoomMessages(req) => self.handle_get_room_messages(req),
            ClientMessage::RequestFriend(req) => self.handle_request_friend(req),
            ClientMessage::AcceptFriend(req) => self.handle_accept_friend(req),
            ClientMessage::GetFriends(req) => self.handle_get_friends(req),
            ClientMessage::SendPrivateMessage(req) => self.handle_send_private_message(req),
            ClientMessage::GetPrivateMessages(req) => self.handle_get_private_messages(req),
            ClientMessage::GetUser(req) => self.handle_get_user(req),
            ClientMessage::SearchUsers(req) => self.handle_search_users(req),
        }
    }

    fn handle_auth(&self, req: AuthRequest) -> ServerMessage {
        let peer = PeerIdentity::new(req.peer_id);
        self.set_current_peer(peer.clone());
        ServerMessage::Auth(AuthResponse {
            success: true,
            peer_id: peer.to_string(),
        })
    }

    fn handle_create_user(&self, req: CreateUserRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut user = User::new(req.username, peer.to_string());
        user.display_name = req.display_name.unwrap_or_default();
        user.bio = req.bio;

        self.store.add_user(user.clone());
        ServerMessage::User(UserResponse { user: Some(user) })
    }

    fn handle_update_user(&self, req: UpdateUserRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut users = self.store.users.write();
        if let Some(user) = users.get_mut(peer.as_str()) {
            if let Some(name) = req.display_name {
                user.display_name = name;
            }
            if let Some(bio) = req.bio {
                user.bio = Some(bio);
            }
            user.updated_at = chrono::Utc::now();
            let updated = user.clone();
            drop(users);
            ServerMessage::User(UserResponse {
                user: Some(updated),
            })
        } else {
            ServerMessage::Error(ErrorResponse::new(404, "User not found"))
        }
    }

    fn handle_create_post(&self, req: CreatePostRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut post = Post::new(peer, req.content);
        post.media_hashes = req.media_hashes;
        post.reply_to = req.reply_to;
        post.repost_of = req.repost_of;
        post.visibility = req.visibility;

        self.store.add_post(post.clone());
        ServerMessage::Post(PostResponse { post: Some(post) })
    }

    fn handle_get_feed(&self, req: GetFeedRequest) -> ServerMessage {
        let peer = PeerIdentity::new(req.peer_id);
        let limit = req.limit.unwrap_or(50) as usize;

        let posts: Vec<Post> = self
            .store
            .posts
            .read()
            .values()
            .filter(|p| {
                if p.visibility == PostVisibility::Public {
                    return true;
                }
                if p.author_id == peer {
                    return true;
                }
                false
            })
            .take(limit)
            .cloned()
            .collect();

        ServerMessage::Feed(FeedResponse { posts })
    }

    fn handle_get_post(&self, req: GetPostRequest) -> ServerMessage {
        let post = self.store.get_post(req.post_id);
        ServerMessage::Post(PostResponse { post })
    }

    fn handle_like_post(&self, req: LikePostRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let peer_str = peer.to_string();
        let mut posts = self.store.posts.write();
        if let Some(post) = posts.get_mut(&req.post_id) {
            if post.likes.contains(&peer_str) {
                post.likes.retain(|id| id != &peer_str);
            } else {
                post.likes.push(peer_str);
            }
            let updated = post.clone();
            drop(posts);
            ServerMessage::Post(PostResponse {
                post: Some(updated),
            })
        } else {
            ServerMessage::Error(ErrorResponse::new(404, "Post not found"))
        }
    }

    fn handle_create_room(&self, req: CreateRoomRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut room = ChatRoom::new(req.name, peer, req.is_group);
        room.description = req.description;
        room.is_public = req.is_public;

        self.store.add_room(room.clone());
        ServerMessage::Room(RoomResponse {
            room: Some(room),
            rooms: None,
        })
    }

    fn handle_get_rooms(&self, _req: GetRoomsRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let rooms: Vec<ChatRoom> = self
            .store
            .rooms
            .read()
            .values()
            .filter(|r| r.members.contains(&peer))
            .cloned()
            .collect();

        ServerMessage::Room(RoomResponse {
            room: None,
            rooms: Some(rooms),
        })
    }

    fn handle_join_room(&self, req: JoinRoomRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut rooms = self.store.rooms.write();
        if let Some(room) = rooms.get_mut(&req.room_id) {
            if !room.members.contains(&peer) {
                room.members.push(peer);
            }
            let updated = room.clone();
            drop(rooms);
            ServerMessage::Room(RoomResponse {
                room: Some(updated),
                rooms: None,
            })
        } else {
            ServerMessage::Error(ErrorResponse::new(404, "Room not found"))
        }
    }

    fn handle_leave_room(&self, req: LeaveRoomRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let mut rooms = self.store.rooms.write();
        if let Some(room) = rooms.get_mut(&req.room_id) {
            room.members.retain(|m| m != &peer);
            ServerMessage::Room(RoomResponse {
                room: None,
                rooms: None,
            })
        } else {
            ServerMessage::Error(ErrorResponse::new(404, "Room not found"))
        }
    }

    fn handle_send_room_message(&self, req: SendRoomMessageRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let rooms = self.store.rooms.read();
        if !rooms.contains_key(&req.room_id) {
            return ServerMessage::Error(ErrorResponse::new(404, "Room not found"));
        }
        drop(rooms);

        let mut msg = ChatMessage::new(req.room_id, peer, req.content);
        msg.media_hashes = req.media_hashes;
        msg.reply_to = req.reply_to;

        self.store.add_message(msg.clone());
        ServerMessage::RoomMessage(RoomMessageResponse {
            message: Some(msg),
            messages: None,
        })
    }

    fn handle_get_room_messages(&self, req: GetRoomMessagesRequest) -> ServerMessage {
        let limit = req.limit.unwrap_or(100) as usize;
        let messages: Vec<ChatMessage> = self
            .store
            .get_room_messages(req.room_id)
            .into_iter()
            .take(limit)
            .collect();

        ServerMessage::RoomMessage(RoomMessageResponse {
            message: None,
            messages: Some(messages),
        })
    }

    fn handle_request_friend(&self, req: RequestFriendRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let addressee = PeerIdentity::new(req.peer_id);
        let friendship = Friendship::new(peer, addressee);

        self.store.request_friendship(friendship.clone());
        ServerMessage::Friend(FriendResponse {
            friendship: Some(friendship),
            friends: None,
        })
    }

    fn handle_accept_friend(&self, req: AcceptFriendRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let requester = PeerIdentity::new(req.peer_id);
        if self.store.accept_friendship(&peer, &requester) {
            ServerMessage::Friend(FriendResponse {
                friendship: None,
                friends: None,
            })
        } else {
            ServerMessage::Error(ErrorResponse::new(404, "Friend request not found"))
        }
    }

    fn handle_get_friends(&self, _req: GetFriendsRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let friends: Vec<String> = self
            .store
            .get_friends(&peer)
            .into_iter()
            .map(|p| p.to_string())
            .collect();

        ServerMessage::Friend(FriendResponse {
            friendship: None,
            friends: Some(friends),
        })
    }

    fn handle_send_private_message(&self, req: SendPrivateMessageRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let recipient = PeerIdentity::new(req.recipient_id);
        let mut msg = PrivateMessage::new(peer, recipient, req.content);
        msg.media_hashes = req.media_hashes;

        self.store.add_private_message(msg.clone());
        ServerMessage::PrivateMessage(PrivateMessageResponse {
            message: Some(msg),
            messages: None,
        })
    }

    fn handle_get_private_messages(&self, req: GetPrivateMessagesRequest) -> ServerMessage {
        let peer = match self.current_peer() {
            Some(p) => p,
            None => return ServerMessage::Error(ErrorResponse::new(401, "Not authenticated")),
        };

        let limit = req.limit.unwrap_or(100) as usize;
        let mut messages = self.store.get_private_messages(&peer);

        if let Some(other_peer) = req.peer_id {
            let other = PeerIdentity::new(other_peer);
            messages.retain(|m| m.sender_id == other || m.recipient_id == other);
        }

        messages.truncate(limit);
        ServerMessage::PrivateMessage(PrivateMessageResponse {
            message: None,
            messages: Some(messages),
        })
    }

    fn handle_get_user(&self, req: GetUserRequest) -> ServerMessage {
        let user = self.store.get_user(&req.peer_id);
        ServerMessage::User(UserResponse { user })
    }

    fn handle_search_users(&self, req: SearchUsersRequest) -> ServerMessage {
        let _limit = req.limit.unwrap_or(20) as usize;
        let _query = req.query.to_lowercase();

        ServerMessage::User(UserResponse { user: None })
    }
}
