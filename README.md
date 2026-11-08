# bajam

A self-hosted trip-tracking server for sharing your location and updates during long journeys (e.g. bike trips). It runs on your own [Tailscale](https://tailscale.com) tailnet so only people on your private network can access it.

## The idea

Instead of using Polarstep, you run your own minimal pipeline:

1. **iPhone Shortcuts** grab your GPS latitude/longitude
2. Connect to **Tailscale**
3. **POST** your position to the server on the tailnet:
   ```json
   {
     "latitude": XX.XXXXX,
     "longitude": XX.XXXXX
   }
   ```
4. A **pin appears on a web map** (powered by [MapLibre GL JS](https://maplibre.org/))
5. Optionally add text descriptions and photos

> "But more.... much more than this... I did it my way..."

## Tech stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2024) |
| Web framework | Axum |
| Runtime | Tokio |
| Database | SQLite via sqlx |
| Auth | bcrypt (passwords) + SHA-256 hashed UUID session tokens (30-day TTL) |
| Templates | Askama |
| Image handling | `image` crate (validation, size limits) |
| Frontend | MapLibre GL JS (CDN), vanilla JS |
| Migrations | sqlx migrate (embedded in binary) |

## Directory layout

```
.
├── Cargo.toml / Cargo.lock        # Rust manifest
├── README.md                      # This file
├── compose.yaml / dockerfile      # Docker setup
├── migrations/                    # SQL migration files
├── src/                           # Rust source
│   ├── main.rs                    # Entry point
│   ├── config.rs                  # CLI args (--tailnet) + env config
│   ├── state.rs                   # App state + DB connection
│   ├── routes/                    # HTTP route handlers
│   ├── models/                    # DB model structs (User, Session, Heartbeat, Escapade)
│   ├── middleware/                # auth + request logger
│   ├── db/                        # SQL query helpers
│   └── tasks/                     # background session cleanup (every 24h)
├── templates/                     # Askama HTML templates (index.html)
├── static/                        # CSS, fonts, favicon
├── uploads/                       # uploaded photos (created at runtime)
├── data/                          # SQLite database (created at runtime)
├── setup/                         # setup crate (workspace member)
└── tests/                         # tests
```

## Quick start

```bash
# Clone the repo
git clone <repo-url>
cd bajam

# 1. Create the SQLite database and a .env file with predefined values
cargo run -p setup

# 2. Launch the server
# By default binds to TCP_LOCAL. Use --tailnet to bind to TCP_TAILNET instead.
cargo run -- --tailnet
```

Required environment variables (in `.env`):
- `DATABASE_PATH` — path to the SQLite database file
- `TCP_LOCAL` — address to bind when not using tailnet (e.g. `0.0.0.0:3000`)
- `TCP_TAILNET` — address to bind when using `--tailnet` (e.g. `100.x.y.z:3000`)

## API endpoints

### Public (no auth)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Serves the map HTML page with the current + historical positions |
| `GET` | `/image/{picture_id}` | Serves an uploaded photo |
| `POST` | `/user/new` | Register a new user (password hashed with bcrypt) |
| `POST` | `/user/login` | Login with username/password → returns a session token |
| `POST` | `/upload/{picture_id}` | Upload a photo (max 10 MB, max 4096×4096 px) |
| `GET` | `/static/*` | Serve static assets |

### Protected (requires `Authorization: Bearer <token>`)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/escapades` | Create a new escapade (trip) |
| `DELETE` | `/escapades/{escapade_id}` | Delete an escapade (admin only, cascades to heartbeats) |
| `POST` | `/escapades/{escapade_id}/heartbeat` | Post a new GPS heartbeat (location update) |
| `DELETE` | `/heartbeats/{heartbeat_id}` | Delete a heartbeat you own |
| `DELETE` | `/user/logout` | Delete your session |

## Core concepts

- **Escapade**: A trip/journey. Users create escapades and become admins of them.
- **Heartbeat**: A location update (latitude, longitude, optional description, optional picture) tied to an escapade and a user.
- **Session**: Auth token issued at login, valid for 30 days. Expired sessions are cleaned up by a background task every 24 hours.
- **Picture**: Uploaded photos stored in `uploads/` and referenced by heartbeats.

## Frontend

The root page (`/`) renders a MapLibre GL JS map showing:
- A **red marker** at your current/latest position (with popup: timestamp, description, optional photo)
- **Gray markers** for historical positions (last 100, excluding the current one)
- A style selector to switch between Globe and Positron map styles