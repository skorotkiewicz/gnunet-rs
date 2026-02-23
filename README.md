# GNUnet Social

Decentralized social media over GNUnet.

## Features

- **Microblog** — Posts, replies, reposts
- **Chat** — Rooms, groups, private messages
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

## Structure

```
src/
├── gnunet/      # GNUnet bindings
├── social/      # Domain models
├── mqtt/        # Server & handler
└── protocol/    # Message types

client/src/
├── hooks/       # WebSocket, state
├── components/  # UI
└── types/       # TypeScript
```

## License

MIT
