import { X, UserPlus, MessageCircle, Copy } from 'lucide-react';
import { useSocial } from '../hooks';

export function Profile() {
  const { profileUser, closeProfile, peerId, friends, requestFriend, sendPrivateMessage, setCurrentRoom, rooms } = useSocial();

  if (!profileUser) return null;

  const isMe = profileUser.id === peerId;
  const isFriend = friends.includes(profileUser.id);

  const handleCopyId = () => {
    navigator.clipboard.writeText(profileUser.id);
  };

  const handleMessage = () => {
    const existingDm = rooms.find(r => !r.is_group && r.members.includes(profileUser.id));
    if (existingDm) {
      setCurrentRoom(existingDm);
    } else {
      sendPrivateMessage(profileUser.id, 'Hi!');
    }
    closeProfile();
  };

  const handleAddFriend = () => {
    requestFriend(profileUser.id);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') closeProfile();
  };

  return (
    <div className="modal-overlay" onClick={closeProfile} onKeyDown={handleKeyDown} role="button" tabIndex={0}>
      <div className="modal profile-modal" onClick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
        <div className="profile-header">
          <div className="profile-avatar">
            {profileUser.display_name?.[0]?.toUpperCase() || profileUser.username[0]?.toUpperCase() || '?'}
          </div>
          <button type="button" className="btn-close" onClick={closeProfile}>
            <X size={20} />
          </button>
        </div>

        <div className="profile-info">
          <h2 className="profile-name">
            {profileUser.display_name || `@${profileUser.username}`}
          </h2>
          {profileUser.display_name && (
            <span className="profile-username">@{profileUser.username}</span>
          )}
          {profileUser.bio && <p className="profile-bio">{profileUser.bio}</p>}
          
          <div className="profile-id">
            <code>{profileUser.id}</code>
            <button type="button" onClick={handleCopyId} title="Copy ID">
              <Copy size={14} />
            </button>
          </div>
        </div>

        {!isMe && (
          <div className="profile-actions">
            <button type="button" className="btn-primary" onClick={handleMessage}>
              <MessageCircle size={16} />
              Message
            </button>
            {!isFriend && (
              <button type="button" className="btn-secondary" onClick={handleAddFriend}>
                <UserPlus size={16} />
                Add Friend
              </button>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
