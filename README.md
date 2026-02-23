# GNUnet Social

Decentralized social media over GNUnet.

**[→ Getting Started Guide](GETTING_STARTED.md)**

## Features

- **Microblog** — Posts, replies, reposts, likes
- **Visibility Control** — Public, Followers, Mutuals, Private
- **User Profiles** — View profiles, bio, peer info
- **Chat Rooms** — Create, join, real-time messaging
- **Private Messages** — Direct messaging with friends
- **Friendship System** — Add friends, accept requests
- **Identity** — GNS zones, peer authentication
- **Transport** — CADET end-to-end encrypted channels

## Architecture

```
┌─────────────────┐     WebSocket      ┌──────────────────┐
│   React Client  │ ◄───────────────► │   Rust Server    │
└─────────────────┘                    └────────┬─────────┘
                                                │
                                       ┌────────▼─────────┐
                                       │     GNUnet       │
                                       │  CADET · GNS     │
                                       └──────────────────┘
```

## Run

```bash
# Server
cargo run

# Client
cd client && bun install && bun run dev
```

Server runs on `ws://localhost:8080`

## Stack

| Layer | Tech |
|-------|------|
| Frontend | React, TypeScript, Vite |
| Transport | WebSocket, MQTT protocol |
| Backend | Rust, Tokio, async |
| Network | GNUnet (CADET, GNS) |

## Protocol

JSON messages over WebSocket:

```json
{ "type": "create_post", "content": "Hello GNUnet!", "media_hashes": [], "visibility": "Public" }
```

<details>
<summary>📁 Project Structure</summary>

```
gnunet-sys/           # GNUnet FFI bindings (bindgen)
├── build.rs          # Auto-generates Rust bindings
├── wrapper.h         # GNUnet headers to bind
└── src/lib.rs        # Exports raw FFI

src/
├── gnunet/           # Safe Rust wrappers
│   ├── crypto.rs     # PeerIdentity, HashCode
│   ├── cadet.rs      # CADET channels
│   ├── gns.rs        # GNS lookups
│   └── identity.rs   # Ego management
├── social/           # Domain models
│   └── mod.rs        # User, Post, ChatRoom, etc.
├── mqtt/             # Server logic
│   ├── server.rs     # WebSocket server
│   └── handler.rs    # Message handlers
└── protocol/         # Message types
    └── messages.rs   # ClientMessage, ServerMessage

client/src/
├── hooks/            # React hooks
│   ├── useSocial.tsx # Global state context
│   └── useWebSocket.tsx
├── components/       # UI components
│   ├── Feed.tsx      # Posts & composer
│   ├── Chat.tsx      # Room messages
│   ├── Sidebar.tsx   # Rooms & friends
│   ├── Profile.tsx   # User profile modal
│   └── Login.tsx     # Peer ID auth
└── types/            # TypeScript types
    └── index.ts      # All interfaces
```
</details>

## Status

🚧 **Alpha** — Core features work, API may change. Perfect for experimentation and contribution!

## Contributing

PRs welcome! Areas that need help:

- GNUnet service integrations (MESSENGER, NAMESTORE)
- Mobile client (React Native?)
- Better error handling & reconnection
- End-to-end encryption
- File/media sharing
- Testing infrastructure

## License

MIT
