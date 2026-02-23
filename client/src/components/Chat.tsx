import { useState, useRef, useEffect } from 'react';
import { useSocial } from '../hooks';

export function Chat() {
  const { currentRoom, roomMessages, sendMessage } = useSocial();
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const prevLengthRef = useRef(0);

  useEffect(() => {
    if (roomMessages.length !== prevLengthRef.current) {
      prevLengthRef.current = roomMessages.length;
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [roomMessages.length]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (input.trim() && currentRoom) {
      sendMessage(currentRoom.id, input.trim());
      setInput('');
    }
  };

  if (!currentRoom) {
    return (
      <div className="chat-empty">
        <p>Select a room to start chatting</p>
      </div>
    );
  }

  const messages = roomMessages.filter((m) => m.room_id === currentRoom.id);

  return (
    <div className="chat">
      <header className="chat-header">
        <h3>{currentRoom.name}</h3>
        <span className="chat-members">{currentRoom.members.length} members</span>
      </header>
      
      <div className="chat-messages">
        {messages.map((msg) => (
          <div key={msg.id} className="chat-message">
            <span className="message-author">@{msg.sender_id.id.slice(0, 8)}</span>
            <span className="message-content">{msg.content}</span>
            <time className="message-time">
              {new Date(msg.created_at).toLocaleTimeString()}
            </time>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>
      
      <form className="chat-input" onSubmit={handleSubmit}>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Type a message..."
        />
        <button type="submit" disabled={!input.trim()}>
          Send
        </button>
      </form>
    </div>
  );
}
