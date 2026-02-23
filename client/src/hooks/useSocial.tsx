import { createContext, useContext, useState, useEffect, useCallback, useRef, type ReactNode } from 'react';
import { useWebSocket } from './useWebSocket';
import type { User, Post, ChatRoom, ChatMessage, PrivateMessage, ClientMessage, ServerMessage, EventMessage } from '../types';

interface SocialContextValue {
  connected: boolean;
  user: User | null;
  posts: Post[];
  rooms: ChatRoom[];
  currentRoom: ChatRoom | null;
  roomMessages: ChatMessage[];
  privateMessages: PrivateMessage[];
  friends: string[];
  send: (msg: ClientMessage) => boolean;
  login: (peerId: string) => void;
  createPost: (content: string) => void;
  joinRoom: (roomId: string) => void;
  sendMessage: (roomId: string, content: string) => void;
  sendPrivateMessage: (recipientId: string, content: string) => void;
  setCurrentRoom: (room: ChatRoom | null) => void;
  requestFriend: (peerId: string) => void;
  getFeed: () => void;
}

const SocialContext = createContext<SocialContextValue | null>(null);

const PEER_ID_KEY = 'gnunet_peer_id';

function getPeerId(): string | null {
  return localStorage.getItem(PEER_ID_KEY);
}

function handleEvent(
  event: EventMessage,
  currentRoom: ChatRoom | null,
  setPosts: React.Dispatch<React.SetStateAction<Post[]>>,
  setRoomMessages: React.Dispatch<React.SetStateAction<ChatMessage[]>>,
  setPrivateMessages: React.Dispatch<React.SetStateAction<PrivateMessage[]>>,
  setFriends: React.Dispatch<React.SetStateAction<string[]>>
) {
  switch (event.event) {
    case 'new_post':
      setPosts(prev => [event.post, ...prev]);
      break;
    case 'new_room_message':
      if (currentRoom?.id === event.room_id) {
        setRoomMessages(prev => [...prev, event.message]);
      }
      break;
    case 'new_private_message':
      setPrivateMessages(prev => [...prev, event.message]);
      break;
    case 'friend_request':
      setFriends(prev => [...prev, event.from]);
      break;
  }
}

export function SocialProvider({ children }: { children: ReactNode }) {
  const [peerId] = useState(getPeerId);
  const wsUrl = `ws://${window.location.hostname}:8080/ws/${peerId}`;

  const { connected, send, subscribe } = useWebSocket(wsUrl);
  const [user, setUser] = useState<User | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [rooms, setRooms] = useState<ChatRoom[]>([]);
  const [currentRoom, setCurrentRoom] = useState<ChatRoom | null>(null);
  const [roomMessages, setRoomMessages] = useState<ChatMessage[]>([]);
  const [privateMessages, setPrivateMessages] = useState<PrivateMessage[]>([]);
  const [friends, setFriends] = useState<string[]>([]);
  const currentRoomRef = useRef(currentRoom);

  useEffect(() => {
    currentRoomRef.current = currentRoom;
  }, [currentRoom]);

  useEffect(() => {
    const unsubscribe = subscribe((msg: ServerMessage) => {
      switch (msg.type) {
        case 'auth':
          if (msg.success) {
            send({ type: 'get_feed', peer_id: msg.peer_id });
          }
          break;
        case 'user':
          if (msg.user) setUser(msg.user);
          break;
        case 'feed':
          setPosts(msg.posts);
          break;
        case 'room':
          if (msg.room) {
            setRooms(prev => {
              if (prev.find(r => r.id === msg.room!.id)) return prev;
              return [...prev, msg.room!];
            });
          }
          break;
        case 'room_message':
          if (msg.message) {
            setRoomMessages(prev => [...prev, msg.message!]);
          } else if (msg.messages) {
            setRoomMessages(msg.messages);
          }
          break;
        case 'private_message':
          if (msg.message) {
            setPrivateMessages(prev => [...prev, msg.message!]);
          } else if (msg.messages) {
            setPrivateMessages(msg.messages);
          }
          break;
        case 'friend':
          if (msg.friends) setFriends(msg.friends);
          break;
        case 'event':
          handleEvent(msg.event, currentRoomRef.current, setPosts, setRoomMessages, setPrivateMessages, setFriends);
          break;
      }
    });
    return unsubscribe;
  }, [subscribe, send]);

  const login = useCallback((newPeerId: string) => {
    localStorage.setItem(PEER_ID_KEY, newPeerId);
    send({ type: 'auth', peer_id: newPeerId });
  }, [send]);

  const createPost = useCallback((content: string) => {
    send({
      type: 'create_post',
      content,
      media_hashes: [],
      visibility: 'Public',
    });
  }, [send]);

  const joinRoom = useCallback((roomId: string) => {
    send({ type: 'join_room', room_id: roomId });
  }, [send]);

  const sendMessage = useCallback((roomId: string, content: string) => {
    send({
      type: 'send_room_message',
      room_id: roomId,
      content,
      media_hashes: [],
    });
  }, [send]);

  const sendPrivateMessage = useCallback((recipientId: string, content: string) => {
    send({
      type: 'send_private_message',
      recipient_id: recipientId,
      content,
      media_hashes: [],
    });
  }, [send]);

  const requestFriend = useCallback((friendPeerId: string) => {
    send({ type: 'request_friend', peer_id: friendPeerId });
  }, [send]);

  const getFeed = useCallback(() => {
    if (user) {
      send({ type: 'get_feed', peer_id: user.id.id });
    }
  }, [send, user]);

  return (
    <SocialContext.Provider
      value={{
        connected,
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
        joinRoom,
        sendMessage,
        sendPrivateMessage,
        setCurrentRoom,
        requestFriend,
        getFeed,
      }}
    >
      {children}
    </SocialContext.Provider>
  );
}

export function useSocial() {
  const ctx = useContext(SocialContext);
  if (!ctx) throw new Error('useSocial must be used within SocialProvider');
  return ctx;
}
