import { Plus, ArrowUpRight, Info, Mail, Copy, Check } from 'lucide-react';
import { useState } from 'react';
import { useSocial } from '../hooks';

export function Sidebar() {
  const { rooms, friends, setCurrentRoom, currentRoom, createRoom, joinRoom, sendPrivateMessage, peerId } = useSocial();
  const [showNewRoom, setShowNewRoom] = useState(false);
  const [showJoinRoom, setShowJoinRoom] = useState(false);
  const [showMessage, setShowMessage] = useState<string | null>(null);
  const [showRoomInfo, setShowRoomInfo] = useState<string | null>(null);
  const [roomName, setRoomName] = useState('');
  const [roomIdInput, setRoomIdInput] = useState('');
  const [message, setMessage] = useState('');
  const [copied, setCopied] = useState(false);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

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
            <button type="button" onClick={() => setShowNewRoom(true)} title="Create room"><Plus size={12} /></button>
            <button type="button" onClick={() => setShowJoinRoom(true)} title="Join room"><ArrowUpRight size={12} /></button>
          </div>
        </div>

        {showNewRoom && (
          <form className="inline-form" onSubmit={handleCreateRoom}>
            <input
              type="text"
              value={roomName}
              onChange={(e) => setRoomName(e.target.value)}
              placeholder="Room name"
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
              placeholder="Room ID to join"
            />
            <button type="submit">Join</button>
            <button type="button" onClick={() => setShowJoinRoom(false)}>×</button>
          </form>
        )}

        <div className="room-list">
          {rooms.map((room) => (
            <div key={room.id} className="room-item-wrapper">
              <button
                className={`room-item ${currentRoom?.id === room.id ? 'active' : ''}`}
                onClick={() => setCurrentRoom(room)}
                type="button"
              >
                <span className="room-name">{room.name}</span>
                <span className="room-members">{room.members.length}</span>
              </button>
              <button
                type="button"
                className="btn-room-info"
                onClick={() => setShowRoomInfo(room.id)}
                title="Room info"
              >
                <Info size={12} />
              </button>
            </div>
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
                <Mail size={12} />
              </button>
            </div>
          ))}
        </div>
      </section>

      <section className="my-info">
        <h3>Your Peer ID</h3>
        <div className="peer-id-box">
          <code>{peerId}</code>
          <button
            type="button"
            onClick={() => peerId && copyToClipboard(peerId)}
            title="Copy peer ID"
          >
            {copied ? <Check size={12} /> : <Copy size={12} />}
          </button>
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
              />
              <div className="modal-actions">
                <button type="submit" disabled={!message.trim()}>Send</button>
                <button type="button" onClick={() => setShowMessage(null)}>Cancel</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showRoomInfo && (() => {
        const room = rooms.find(r => r.id === showRoomInfo);
        if (!room) return null;
        return (
          <div className="modal-overlay" onClick={() => setShowRoomInfo(null)}>
            <div className="modal" onClick={(e) => e.stopPropagation()}>
              <h4>{room.name}</h4>
              <div className="room-info-field">
                <label>Room ID:</label>
                <div className="copy-field">
                  <code>{room.id}</code>
                  <button
                    type="button"
                    onClick={() => copyToClipboard(room.id)}
                  >
                    {copied ? <Check size={12} /> : <Copy size={12} />}
                  </button>
                </div>
              </div>
              <div className="room-info-field">
                <label>Owner:</label>
                <code>{room.owner_id.slice(0, 16)}...</code>
              </div>
              <div className="room-info-field">
                <label>Members:</label>
                <span>{room.members.length}</span>
              </div>
              <div className="modal-actions">
                <button type="button" onClick={() => setShowRoomInfo(null)}>Close</button>
              </div>
            </div>
          </div>
        );
      })()}
    </aside>
  );
}
