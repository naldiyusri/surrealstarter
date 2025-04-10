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

**You may need to modify some elements to fit your preferences and project.**

```

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
```bash
surreal start --user root --pass root memory
```

4. Run the API

```bash
cargo run
```

---

# Rewrite it in Rust.
