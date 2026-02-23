import { useState } from 'react';
import { useSocial } from '../hooks';

export function Sidebar() {
  const { rooms, friends, setCurrentRoom, currentRoom, createRoom, joinRoom, sendPrivateMessage } = useSocial();
  const [showNewRoom, setShowNewRoom] = useState(false);
  const [showJoinRoom, setShowJoinRoom] = useState(false);
  const [showMessage, setShowMessage] = useState<string | null>(null);
  const [roomName, setRoomName] = useState('');
  const [roomIdInput, setRoomIdInput] = useState('');
  const [message, setMessage] = useState('');

  const handleCreateRoom = (e: React.FormEvent) => {
    e.preventDefault();
    if (roomName.trim()) {
      createRoom(roomName.trim(), true, true);
      setRoomName('');
      setShowNewRoom(false);
    }
  };

  const handleJoinRoom = (e: React.FormEvent) => {
    e.preventDefault();
    if (roomIdInput.trim()) {
      joinRoom(roomIdInput.trim());
      setRoomIdInput('');
      setShowJoinRoom(false);
    }
  };

  const handleSendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (message.trim() && showMessage) {
      sendPrivateMessage(showMessage, message.trim());
      setMessage('');
      setShowMessage(null);
    }
  };

  return (
    <aside className="sidebar">
      <section className="rooms-section">
        <div className="section-header">
          <h3>Rooms</h3>
          <div className="section-actions">
            <button type="button" onClick={() => setShowNewRoom(true)} title="Create room">+</button>
            <button type="button" onClick={() => setShowJoinRoom(true)} title="Join room">↗</button>
          </div>
        </div>

        {showNewRoom && (
          <form className="inline-form" onSubmit={handleCreateRoom}>
            <input
              type="text"
              value={roomName}
              onChange={(e) => setRoomName(e.target.value)}
              placeholder="Room name"
              autoFocus
            />
            <button type="submit">Create</button>
            <button type="button" onClick={() => setShowNewRoom(false)}>×</button>
          </form>
        )}

        {showJoinRoom && (
          <form className="inline-form" onSubmit={handleJoinRoom}>
            <input
              type="text"
              value={roomIdInput}
              onChange={(e) => setRoomIdInput(e.target.value)}
              placeholder="Room ID"
              autoFocus
            />
            <button type="submit">Join</button>
            <button type="button" onClick={() => setShowJoinRoom(false)}>×</button>
          </form>
        )}

        <div className="room-list">
          {rooms.map((room) => (
            <button
              key={room.id}
              className={`room-item ${currentRoom?.id === room.id ? 'active' : ''}`}
              onClick={() => setCurrentRoom(room)}
              type="button"
            >
              <span className="room-name">{room.name}</span>
              <span className="room-members">{room.members.length}</span>
            </button>
          ))}
        </div>
      </section>

      <section className="friends-section">
        <h3>Friends ({friends.length})</h3>
        <div className="friend-list">
          {friends.map((friendId) => (
            <div key={friendId} className="friend-item">
              <span className="friend-avatar">
                {friendId.slice(0, 2).toUpperCase()}
              </span>
              <span className="friend-name">{friendId.slice(0, 12)}</span>
              <button
                type="button"
                className="btn-message"
                onClick={() => setShowMessage(friendId)}
                title="Send message"
              >
                ✉
              </button>
            </div>
          ))}
        </div>
      </section>

      {showMessage && (
        <div className="modal-overlay" onClick={() => setShowMessage(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h4>Message to @{showMessage.slice(0, 8)}</h4>
            <form onSubmit={handleSendMessage}>
              <textarea
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                placeholder="Type your message..."
                rows={3}
                autoFocus
              />
              <div className="modal-actions">
                <button type="submit" disabled={!message.trim()}>Send</button>
                <button type="button" onClick={() => setShowMessage(null)}>Cancel</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </aside>
  );
}
