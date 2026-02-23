import { useState } from 'react';
import { useSocial } from '../hooks';

interface LoginProps {
  onLogin?: () => void;
}

export function Login({ onLogin }: LoginProps) {
  const { login, connected } = useSocial();
  const [peerId, setPeerId] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (peerId.trim()) {
      login(peerId.trim());
      onLogin?.();
    }
  };

  return (
    <div className="login">
      <div className="login-card">
        <h1>GNUnet Social</h1>
        <p className="login-subtitle">Decentralized social networking</p>

        {!connected ? (
          <div className="login-connecting">Connecting to network...</div>
        ) : (
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              value={peerId}
              onChange={(e) => setPeerId(e.target.value)}
              placeholder="Enter your peer ID"
            />
            <button type="submit" disabled={!peerId.trim()}>
              Connect
            </button>
          </form>
        )}
      </div>
    </div>
  );
}
