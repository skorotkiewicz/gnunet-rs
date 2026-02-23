import { Chat, Feed, Login, Profile, Sidebar } from "./components";
import { SocialProvider, useSocial } from "./hooks";
import "./App.css";

function AppContent() {
	const { authenticated, peerId } = useSocial();

	if (!authenticated || !peerId) {
		return <Login />;
	}

	return (
		<>
			<div className="app-layout">
				<Sidebar />
				<main className="main-content">
					<Feed />
				</main>
				<aside className="chat-panel">
					<Chat />
				</aside>
			</div>
			<Profile />
		</>
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
