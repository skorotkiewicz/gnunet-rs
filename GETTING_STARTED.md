# Getting Started with GNUnet Social

A step-by-step guide to set up and run your decentralized social network.

## Prerequisites

### 1. Install GNUnet

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install gnunet
```

**Arch Linux:**
```bash
yay -S gnunet
# or 
paru -S gnunet
```

**From Source:**
See [GNUnet Installation Guide](https://www.gnunet.org/en/install.html)

### 2. Start GNUnet Services

```bash
# Start GNUnet peer
gnunet-arm -s

# Verify services are running
gnunet-arm -I
```

You should see services like `cadet`, `gns`, `identity`, `dht` listed.

### 3. Get Your Peer Identity

```bash
gnunet-identity -d
```

This will show your peer ID (e.g., `SXE8KY6W87Y34BWS06S7VT23MPBQKV6ZA7R3NKV5R4EWA7N5TSNG`). You'll use this to log in.

### 4. Install Build Tools

**Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Bun (for client):**
```bash
curl -fsSL https://bun.sh/install | bash
source ~/.bashrc
```

## Running the Server

```bash
# Clone the repository
git clone https://github.com/skorotkiewicz/gnunet-rs.git
cd gnunet-rs

# Build and run
cargo run
```

The server will start on `ws://localhost:8080`.

You should see GNUnet debug output showing CADET messages being exchanged.

## Running the Client

```bash
# In a new terminal
cd client

# Install dependencies
bun install

# Start development server
bun run dev
```

Open http://localhost:5173 in your browser.

## First Login

1. Get your peer ID from `gnunet-identity -d`
2. Enter it in the login screen
3. You're now connected!

## Features

### Posts
- Write posts with visibility: Public, Followers, Mutuals, Private
- Like and repost
- Click on author names to view profiles

### Chat Rooms
- Create public or private rooms
- Join rooms by room ID
- Real-time messaging

### Friends
- Send friend requests to other peers
- Accept incoming requests
- Send private messages to friends

### Profiles
- Click any username to view their profile
- Send messages or friend requests from profiles

## Troubleshooting

### "GNUnet service not found"

Make sure GNUnet is running:
```bash
gnunet-arm -s
gnunet-arm -I
```

### "Peer ID not accepted"

Check your peer identity exists:
```bash
gnunet-identity -d
```

If empty, create an ego:
```bash
gnunet-identity -C "my-ego"
```

### "Connection refused" errors

1. Verify server is running on port 8080
2. Check firewall allows localhost connections
3. Ensure GNUnet ARM is started

### "CADET channel failed"

CADET requires time to establish connections with peers. Wait a few seconds and try again.

### "No posts appearing"

- Make sure you're authenticated
- Check browser console for WebSocket errors
- Verify server logs show message processing

## Architecture

```
Browser (React) ──WebSocket──► Rust Server ──CADET──► GNUnet P2P Network
                                      │
                                      ├── GNS (names)
                                      ├── Identity (auth)
                                      └── DHT (discovery)
```

## Development

### Build Client for Production

```bash
cd client
bun run build
```

Production files go to `client/dist/`.

### Server Configuration

The server runs on `0.0.0.0:8080` by default. WebSocket endpoint is `/ws`.

### Protocol

All messages are JSON over WebSocket:

```typescript
// Client → Server
{ type: "auth", peer_id: "YOUR_PEER_ID" }
{ type: "create_post", content: "Hello!", media_hashes: [], visibility: "Public" }
{ type: "like_post", post_id: "abc123" }

// Server → Client
{ type: "auth", success: true, peer_id: "YOUR_PEER_ID" }
{ type: "post", post: { id: "...", content: "...", likes: [], ... } }
{ type: "feed", posts: [...] }
```

## Need Help?

- [GNUnet Documentation](https://docs.gnunet.org/latest/)
- [GNUnet Handbook](https://docs.gnunet.org/doxygen/index.html)
- Check server logs for debug output
- Browser DevTools → Network → WS tab for WebSocket messages
