## About
 blazing-fast Axum API starter powered by SurrealDB — perfect for building modern backends with minimal setup and maximum performance.

This starter includes everything you need to hit the ground running:
- Axum — ergonomic, modular web framework
- SurrealDB — https://surrealdb.com/
- Oauth2 (Discord) — classic user authentication via Discord
- CORS — cross-origin resource sharing
- Middleware — auth session handling
- Error Handling — consistent and structured error using `anyhow` `thiserror`
- Session Management — secure, cookie-based sessions
- Rate Limiting — basic rate control using `tower-governor` 
- Tracing — request tracing using `tracing` and `tracing-subscriber`
- Docker — containerized environment for easy deployment 
- Caddyfile — basic Caddy configuration for web service
 
***Edit the `Caddyfile` to set your domain.**

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

Memory
```bash
surreal start --user root --pass root memory
```
Visualize it with Surrealist:
https://surrealdb.com/docs/surrealist/installation

Remote (1 node free): https://surrealist.app/

4. Run the API

Dev
```bash
cargo watch -x run
```

Docker
```bash
docker compose up -d
```

---

# Rewrite it in Rust.
