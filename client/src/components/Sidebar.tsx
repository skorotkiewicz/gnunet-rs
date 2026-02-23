import { useSocial } from '../hooks';

export function Sidebar() {
  const { rooms, friends, setCurrentRoom, currentRoom } = useSocial();

  return (
    <aside className="sidebar">
      <section className="rooms-section">
        <h3>Rooms</h3>
        <div className="room-list">
          {rooms.map((room) => (
            <button
              key={room.id}
              className={`room-item ${currentRoom?.id === room.id ? 'active' : ''}`}
              onClick={() => setCurrentRoom(room)}
              type="button"
            >
              <span className="room-name">{room.name}</span>
              {room.is_group && <span className="room-badge">Group</span>}
            </button>
          ))}
        </div>
      </section>
      
      <section className="friends-section">
        <h3>Friends ({friends.length})</h3>
        <div className="friend-list">
          {friends.map((peerId) => (
            <div key={peerId} className="friend-item">
              <span className="friend-avatar">
                {peerId.slice(0, 2).toUpperCase()}
              </span>
              <span className="friend-name">{peerId.slice(0, 12)}</span>
            </div>
          ))}
        </div>
      </section>
    </aside>
  );
}
