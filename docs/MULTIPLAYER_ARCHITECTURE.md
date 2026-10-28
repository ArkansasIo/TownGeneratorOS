# Procedural Universe and Multiplayer Architecture

## Current implementation status

- `galaxy.html` is a browser-side Three.js prototype with seeded star systems, four selectable quadrants, solar-system orbit views, spherical planet views, light/shading, and procedural game stats.
- `server/` is an initial Rust/Axum service scaffold with MySQL migrations, a health endpoint, universe metadata endpoint, and a WebSocket transport test.
- Multiplayer is **not production-ready**: player registration/login, password hashing, authenticated WebSocket sessions, authoritative simulation commands, conflict resolution, and live deployment are still required. The current socket explicitly returns a notice rather than applying game actions.

## Universe addressing

The prototype uses four quadrants and 499 logical sectors per quadrant. This is a navigation scheme, not a claim that the entire observable universe has been catalogued. Every generated object must be reproducible from a stable universe seed plus hierarchical identifiers:

`universe seed → quadrant → sector → system → star → planet → moon → surface region`

Store generation version alongside seeds so generator upgrades do not silently change existing worlds.

## Service boundaries

- **Rust authoritative simulation:** validates player actions, advances ticks, owns combat/economy/AI, resolves simultaneous commands and emits durable events.
- **MySQL:** accounts, universe metadata, object identifiers, saved settlements, ownership, durable events, and selected scientific records. Avoid persisting every frame; store state checkpoints and append meaningful events.
- **WebSocket:** authenticated real-time updates and player commands. Validate message size/schema, rate limits, ownership and action permissions.
- **Three.js client:** rendering, camera controls, local previews and interpolation only. It must not be trusted to decide combat, ownership, inventory or resource changes.

## Scientific versus generated values

Keep verified scientific measurements and their source/license/provenance separate from procedural game attributes. Scientific values should include units, uncertainty where relevant, source citation, retrieval/version date and a flag indicating measured, derived or fictional. Never label random procedural radii, atmospheres, terrain or orbital periods as real scientific data.

## Local development

1. Install Rust stable and MySQL 8.
2. Create a database and restricted database user; grant access only to the project database.
3. Copy `server/.env.example` to `server/.env` and set `DATABASE_URL` and `BIND_ADDR`. Do not commit secrets.
4. Run `mysql < server/migrations/001_init.sql` only if you prefer manual schema setup; otherwise the Rust service applies the migration directory at startup.
5. Run `cd server && cargo run`; check `GET /health` and `GET /api/v1/universe`.
6. Open `galaxy.html` through GitHub Pages or a local static web server. The renderer loads Three.js from jsDelivr, so network access and WebGL are required.

## Before public multiplayer launch

Implement Argon2id password hashes, HTTPS, session/token expiry and revocation, authenticated WebSocket upgrades, input validation, rate limiting, per-universe authoritative tick locks, reconnect/resume, backups, migration tests, integration tests, abuse controls, and server-side ownership checks. Replace permissive CORS with explicit production origins. Deploy the Rust service on a server/container platform; GitHub Pages can host the static client but cannot run the Rust process or MySQL database.
