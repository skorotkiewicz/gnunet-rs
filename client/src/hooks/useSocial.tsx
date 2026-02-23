import {
	createContext,
	type ReactNode,
	useCallback,
	useContext,
	useEffect,
	useRef,
	useState,
} from "react";
import type {
	ChatMessage,
	ChatRoom,
	ClientMessage,
	EventMessage,
	Post,
	PrivateMessage,
	ServerMessage,
	User,
} from "../types";
import { useWebSocket } from "./useWebSocket";

export interface SocialContextValue {
	connected: boolean;
	authenticated: boolean;
	peerId: string | null;
	user: User | null;
	posts: Post[];
	rooms: ChatRoom[];
	currentRoom: ChatRoom | null;
	roomMessages: ChatMessage[];
	privateMessages: PrivateMessage[];
	friends: string[];
	send: (msg: ClientMessage) => boolean;
	login: (peerId: string) => void;
	createPost: (content: string, visibility?: string) => void;
	likePost: (postId: string) => void;
	createRoom: (name: string, isGroup: boolean, isPublic: boolean) => void;
	getRooms: () => void;
	joinRoom: (roomId: string) => void;
	sendMessage: (roomId: string, content: string) => void;
	sendPrivateMessage: (recipientId: string, content: string) => void;
	setCurrentRoom: (room: ChatRoom | null) => void;
	requestFriend: (peerId: string) => void;
	acceptFriend: (peerId: string) => void;
	getFeed: () => void;
	getFriends: () => void;
	getUser: (peerId: string) => void;
	getRoomMessages: (roomId: string) => void;
	users: Map<string, User>;
	showProfile: (peerId: string) => void;
	profileUser: User | null;
	closeProfile: () => void;
}

const SocialContext = createContext<SocialContextValue | null>(null);

const PEER_ID_KEY = "gnunet_peer_id";

function getStoredPeerId(): string | null {
	return localStorage.getItem(PEER_ID_KEY);
}

function handleEvent(
	event: EventMessage,
	currentRoom: ChatRoom | null,
	setPosts: React.Dispatch<React.SetStateAction<Post[]>>,
	setRoomMessages: React.Dispatch<React.SetStateAction<ChatMessage[]>>,
	setPrivateMessages: React.Dispatch<React.SetStateAction<PrivateMessage[]>>,
	setFriends: React.Dispatch<React.SetStateAction<string[]>>,
) {
	switch (event.event) {
		case "new_post":
			setPosts((prev) => [event.post, ...prev]);
			break;
		case "new_room_message":
			if (currentRoom?.id === event.room_id) {
				setRoomMessages((prev) => [...prev, event.message]);
			}
			break;
		case "new_private_message":
			setPrivateMessages((prev) => [...prev, event.message]);
			break;
		case "friend_request":
			setFriends((prev) =>
				prev.includes(event.from) ? prev : [...prev, event.from],
			);
			break;
		case "friend_accepted":
			setFriends((prev) =>
				prev.includes(event.peer_id) ? prev : [...prev, event.peer_id],
			);
			break;
	}
}

export function SocialProvider({ children }: { children: ReactNode }) {
	const wsUrl = `ws://${window.location.hostname}:8080/ws`;

	const { connected, send, subscribe } = useWebSocket(wsUrl);
	const [authenticated, setAuthenticated] = useState(false);
	const [peerId, setPeerId] = useState<string | null>(getStoredPeerId);
	const [user, setUser] = useState<User | null>(null);
	const [users, setUsers] = useState<Map<string, User>>(new Map());
	const [posts, setPosts] = useState<Post[]>([]);
	const [rooms, setRooms] = useState<ChatRoom[]>([]);
	const [currentRoom, setCurrentRoom] = useState<ChatRoom | null>(null);
	const [roomMessages, setRoomMessages] = useState<ChatMessage[]>([]);
	const [privateMessages, setPrivateMessages] = useState<PrivateMessage[]>([]);
	const [friends, setFriends] = useState<string[]>([]);
	const [profileUser, setProfileUser] = useState<User | null>(null);
	const currentRoomRef = useRef(currentRoom);

	useEffect(() => {
		currentRoomRef.current = currentRoom;
	}, [currentRoom]);

	useEffect(() => {
		if (connected && peerId && !authenticated) {
			send({ type: "auth", peer_id: peerId });
		}
	}, [connected, peerId, authenticated, send]);

	useEffect(() => {
		const unsubscribe = subscribe((msg: ServerMessage) => {
			switch (msg.type) {
				case "auth":
					if (msg.success) {
						setAuthenticated(true);
						send({ type: "get_feed", peer_id: msg.peer_id });
						send({ type: "get_rooms" });
						send({ type: "get_friends" });
					}
					break;
				case "user":
					if (msg.user) {
						setUser(msg.user);
						setUsers((prev) => new Map(prev).set(msg.user!.id, msg.user!));
					}
					break;
				case "feed":
					setPosts(msg.posts);
					break;
				case "post":
					if (msg.post) {
						setPosts((prev) => {
							const idx = prev.findIndex((p) => p.id === msg.post!.id);
							if (idx >= 0) {
								const updated = [...prev];
								updated[idx] = msg.post!;
								return updated;
							}
							return [msg.post!, ...prev];
						});
					}
					break;
				case "room":
					if (msg.rooms) {
						setRooms(msg.rooms);
					} else if (msg.room) {
						setRooms((prev) => {
							if (prev.find((r) => r.id === msg.room!.id)) {
								return prev.map((r) => (r.id === msg.room!.id ? msg.room! : r));
							}
							return [...prev, msg.room!];
						});
					}
					break;
				case "room_message":
					if (msg.message) {
						setRoomMessages((prev) => [...prev, msg.message!]);
					} else if (msg.messages) {
						setRoomMessages(msg.messages);
					}
					break;
				case "private_message":
					if (msg.message) {
						setPrivateMessages((prev) => [...prev, msg.message!]);
					} else if (msg.messages) {
						setPrivateMessages(msg.messages);
					}
					break;
				case "friend":
					if (msg.friends) setFriends(msg.friends);
					break;
				case "event":
					handleEvent(
						msg.event,
						currentRoomRef.current,
						setPosts,
						setRoomMessages,
						setPrivateMessages,
						setFriends,
					);
					break;
			}
		});
		return unsubscribe;
	}, [subscribe, send]);

	const login = useCallback((newPeerId: string) => {
		localStorage.setItem(PEER_ID_KEY, newPeerId);
		setPeerId(newPeerId);
	}, []);

	const createPost = useCallback(
		(content: string, visibility: string = "Public") => {
			send({
				type: "create_post",
				content,
				media_hashes: [],
				visibility,
			});
		},
		[send],
	);

	const likePost = useCallback(
		(postId: string) => {
			send({ type: "like_post", post_id: postId });
		},
		[send],
	);

	const createRoom = useCallback(
		(name: string, isGroup: boolean, isPublic: boolean) => {
			send({
				type: "create_room",
				name,
				is_group: isGroup,
				is_public: isPublic,
			});
		},
		[send],
	);

	const getRooms = useCallback(() => {
		send({ type: "get_rooms" });
	}, [send]);

	const joinRoom = useCallback(
		(roomId: string) => {
			send({ type: "join_room", room_id: roomId });
		},
		[send],
	);

	const sendMessage = useCallback(
		(roomId: string, content: string) => {
			send({
				type: "send_room_message",
				room_id: roomId,
				content,
				media_hashes: [],
			});
		},
		[send],
	);

	const sendPrivateMessage = useCallback(
		(recipientId: string, content: string) => {
			send({
				type: "send_private_message",
				recipient_id: recipientId,
				content,
				media_hashes: [],
			});
		},
		[send],
	);

	const requestFriend = useCallback(
		(friendPeerId: string) => {
			send({ type: "request_friend", peer_id: friendPeerId });
		},
		[send],
	);

	const acceptFriend = useCallback(
		(friendPeerId: string) => {
			send({ type: "accept_friend", peer_id: friendPeerId });
		},
		[send],
	);

	const getFeed = useCallback(() => {
		if (peerId) {
			send({ type: "get_feed", peer_id: peerId });
		}
	}, [send, peerId]);

	const getFriends = useCallback(() => {
		send({ type: "get_friends" });
	}, [send]);

	const getUser = useCallback(
		(targetPeerId: string) => {
			send({ type: "get_user", peer_id: targetPeerId });
		},
		[send],
	);

	const getRoomMessages = useCallback(
		(roomId: string) => {
			send({ type: "get_room_messages", room_id: roomId });
		},
		[send],
	);

	const showProfile = useCallback(
		(targetPeerId: string) => {
			send({ type: "get_user", peer_id: targetPeerId });
			const existing = users.get(targetPeerId);
			if (existing) {
				setProfileUser(existing);
			} else {
				setProfileUser({
					id: targetPeerId,
					username: targetPeerId.slice(0, 8),
					display_name: "",
					bio: null,
					avatar_hash: null,
					gns_zone: "",
					created_at: "",
					updated_at: "",
				});
			}
		},
		[send, users],
	);

	const closeProfile = useCallback(() => {
		setProfileUser(null);
	}, []);

	useEffect(() => {
		if (profileUser?.id && users.has(profileUser.id)) {
			setProfileUser(users.get(profileUser.id)!);
		}
	}, [users, profileUser?.id]);

	return (
		<SocialContext.Provider
			value={{
				connected,
				authenticated,
				peerId,
				user,
				posts,
				rooms,
				currentRoom,
				roomMessages,
				privateMessages,
				friends,
				send,
				login,
				createPost,
				likePost,
				createRoom,
				getRooms,
				joinRoom,
				sendMessage,
				sendPrivateMessage,
				setCurrentRoom,
				requestFriend,
				acceptFriend,
				getFeed,
				getFriends,
				getUser,
				getRoomMessages,
				users,
				showProfile,
				profileUser,
				closeProfile,
			}}
		>
			{children}
		</SocialContext.Provider>
	);
}

export function useSocial() {
	const ctx = useContext(SocialContext);
	if (!ctx) throw new Error("useSocial must be used within SocialProvider");
	return ctx;
}
