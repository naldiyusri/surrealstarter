## About
 blazing-fast Axum API starter powered by SurrealDB — perfect for building modern backends with minimal setup and maximum performance.

This starter includes everything you need to hit the ground running:
- Axum
- SurrealDB
- Oauth2 (Discord)
- CORS
- Middleware
- Error Handling
- Session Management
- Rate Limit
- Tracing
- Docker
- Caddyfile

***You can edit the `Caddyfile` to set your domain.**
**You may need to modify some elements (port, project name...) to fit your preferences.**

## Getting Started

1. Clone the repo

```bash
git clone https://github.com/cpasneedles/surrealstarter
cd surrealstarter
```

2. Set up environment variables

Create a .env file in the root directory and configure the values:
```env
# SurrealDB
SURREAL_ADDRESS=
SURREAL_USERNAME=
SURREAL_PASSWORD=

# Discord Oauth2
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
DISCORD_REDIRECT_URI=
```

3. Run SurrealDB locally (optional)

If you're not using a remote DB:

Memory
```bash
surreal start --user root --pass root memory
```

Remote (free 1 node)
https://surrealist.app/

4. Run the API

Dev
```bash
cargo run
```

Docker
```bash
docker compose up -d
```

---

# Rewrite it in Rust.
