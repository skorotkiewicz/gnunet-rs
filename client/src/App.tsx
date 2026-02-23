import { useState, useEffect } from 'react';
import { SocialProvider, useSocial } from './hooks';
import { Feed, Chat, Sidebar, Login } from './components';
import './App.css';

function AppContent() {
  const { connected } = useSocial();
  const [hasPeerId, setHasPeerId] = useState(() => !!localStorage.getItem('gnunet_peer_id'));

  useEffect(() => {
    const checkPeerId = () => {
      setHasPeerId(!!localStorage.getItem('gnunet_peer_id'));
    };
    window.addEventListener('storage', checkPeerId);
    return () => window.removeEventListener('storage', checkPeerId);
  }, []);

  if (!connected || !hasPeerId) {
    return <Login onLogin={() => setHasPeerId(true)} />;
  }

  return (
    <div className="app-layout">
      <Sidebar />
      <main className="main-content">
        <Feed />
      </main>
      <aside className="chat-panel">
        <Chat />
      </aside>
    </div>
  );
}

function App() {
  return (
    <SocialProvider>
      <AppContent />
    </SocialProvider>
  );
}

export default App;
