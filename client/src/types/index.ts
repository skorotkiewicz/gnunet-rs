export interface User {
  id: string;
  username: string;
  display_name: string;
  bio: string | null;
  avatar_hash: string | null;
  gns_zone: string;
  created_at: string;
  updated_at: string;
}

export type PostVisibility = 'Public' | 'FollowersOnly' | 'MutualsOnly' | 'Private';

export interface Post {
  id: string;
  author_id: string;
  content: string;
  media_hashes: string[];
  reply_to: string | null;
  repost_of: string | null;
  visibility: PostVisibility;
  created_at: string;
  likes: string[];
  reposts: number;
}

export interface ChatRoom {
  id: string;
  name: string;
  description: string | null;
  owner_id: string;
  admins: string[];
  members: string[];
  is_group: boolean;
  is_public: boolean;
  created_at: string;
}

export interface ChatMessage {
  id: string;
  room_id: string;
  sender_id: string;
  content: string;
  media_hashes: string[];
  reply_to: string | null;
  created_at: string;
}

export interface PrivateMessage {
  id: string;
  sender_id: string;
  recipient_id: string;
  content: string;
  media_hashes: string[];
  created_at: string;
  read_at: string | null;
}

export interface Friendship {
  requester_id: string;
  addressee_id: string;
  status: 'Pending' | 'Accepted' | 'Blocked';
  created_at: string;
  updated_at: string;
}

export type ClientMessage =
  | { type: 'auth'; peer_id: string; token?: string }
  | { type: 'create_user'; username: string; display_name?: string; bio?: string }
  | { type: 'update_user'; display_name?: string; bio?: string }
  | { type: 'create_post'; content: string; media_hashes: string[]; reply_to?: string; repost_of?: string; visibility: string }
  | { type: 'get_feed'; peer_id: string; limit?: number; before?: string }
  | { type: 'get_post'; post_id: string }
  | { type: 'like_post'; post_id: string; unlike?: boolean }
  | { type: 'create_room'; name: string; description?: string; is_group: boolean; is_public: boolean }
  | { type: 'get_rooms' }
  | { type: 'join_room'; room_id: string }
  | { type: 'leave_room'; room_id: string }
  | { type: 'send_room_message'; room_id: string; content: string; media_hashes: string[]; reply_to?: string }
  | { type: 'get_room_messages'; room_id: string; limit?: number; before?: string }
  | { type: 'request_friend'; peer_id: string }
  | { type: 'accept_friend'; peer_id: string }
  | { type: 'get_friends' }
  | { type: 'send_private_message'; recipient_id: string; content: string; media_hashes: string[] }
  | { type: 'get_private_messages'; peer_id?: string; limit?: number }
  | { type: 'get_user'; peer_id: string }
  | { type: 'search_users'; query: string; limit?: number };

export type ServerMessage =
  | { type: 'auth'; success: boolean; peer_id: string }
  | { type: 'user'; user: User | null }
  | { type: 'post'; post: Post | null }
  | { type: 'feed'; posts: Post[] }
  | { type: 'room'; room: ChatRoom | null; rooms?: ChatRoom[] }
  | { type: 'room_message'; message: ChatMessage | null; messages?: ChatMessage[] }
  | { type: 'friend'; friendship: Friendship | null; friends?: string[] }
  | { type: 'private_message'; message: PrivateMessage | null; messages?: PrivateMessage[] }
  | { type: 'error'; code: number; message: string }
  | { type: 'event'; event: EventMessage };

export type EventMessage =
  | { event: 'new_post'; post: Post }
  | { event: 'new_room_message'; room_id: string; message: ChatMessage }
  | { event: 'new_private_message'; message: PrivateMessage }
  | { event: 'friend_request'; from: string; friendship: Friendship }
  | { event: 'friend_accepted'; peer_id: string }
  | { event: 'user_online'; peer_id: string }
  | { event: 'user_offline'; peer_id: string };
