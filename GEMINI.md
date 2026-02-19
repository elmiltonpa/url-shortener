# Always use Context7 MCP when I need library/API documentation, code generation, setup or configuration steps without me having to explicitly ask.

# Project Overview: URL Shortener (Rust)

*Note: This project is currently in early development. The core structure is established, but several modules are in the process of being implemented.*

A high-performance URL shortening service built with Rust, focusing on scalability and reliability.

## Core Technologies
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (Tokio ecosystem)
- **Database:** PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) for asynchronous, type-safe queries.
- **Caching:** Redis for fast URL resolution.
- **Runtime:** [Tokio](https://tokio.rs/) for asynchronous I/O.
- **Serialization:** [Serde](https://serde.rs/) for JSON handling.
- **Validation:** [validator](https://github.com/Keats/validator) for input validation.
- **Logging/Tracing:** `tracing` and `tracing-subscriber`.

## Architecture
The project follows a layered architecture:
- `src/handlers/`: HTTP endpoint logic (Create, Redirect, Stats).
- `src/models/`: Data structures for database and API requests/responses.
- `src/repository/`: Data access layer for database interactions.
- `src/services/`: Business logic including short code generation, caching, and validation.
- `src/middleware/`: Custom middleware (e.g., rate limiting).
- `src/config.rs`: Environment-based configuration.
- `src/error.rs`: Centralized error handling using `thiserror`.

## Building and Running

### Prerequisites
- Rust (latest stable)
- PostgreSQL
- Redis
- `sqlx-cli` (optional, for migrations)

### Commands
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`
- **Database Migrations:** `sqlx migrate run` (requires `DATABASE_URL` in `.env`)

## Roadmap & Progress
- [x] **Project Discovery**: Analyze structure and dependencies.
- [x] **Data Models**: Defined `UrlModel`, `UrlRequest`, and `UrlResponse` in `src/models/url.rs`.
- [x] **Error Handling**: Implemented centralized `AppError` with `thiserror` and Axum integration in `src/error.rs`.
- [x] **Repository Layer**: Implemented database interactions (PostgreSQL) in `src/repository/url_repo.rs`, including create, find, and click count incrementing.
- [x] **Business Logic (Services)**:
    - [x] **Short code generation (nanoid strategy)**:
        1. **Alphabet**: Alphanumeric `[0-9a-zA-Z]` (62 chars).
        2. **Length**: Default to 8 characters.
        3. **Collision Strategy**: Retries up to 4 times with `nanoid`.
    - [ ] **Redis caching**: `src/services/cache.rs` is currently a placeholder.
    - [x] **URL validation**: Implemented in `src/services/validator.rs` (checks format, protocol, and self-referencing).
- [ ] **API Handlers**:
    - [x] `POST /`: Create shortened URL (Implemented in `src/handlers/create.rs`).
    - [x] `GET /:code`: Redirect to original URL (Implemented in `src/handlers/redirect.rs`).
    - [ ] `GET /stats/:code`: View URL statistics (`src/handlers/stats.rs` is currently a placeholder).
- [x] **Server Setup**: Wire everything together in `main.rs` with Axum and SQLx.
- [ ] **Middleware**:
    - [ ] **Rate Limiting**: `src/middleware/rate_limit.rs` is currently a placeholder.

## Development Conventions
- **Asynchronous First:** All I/O operations should be `async` using `tokio`.
- **Error Handling:** Use the custom `AppError` type in `src/error.rs`. Prefer `anyhow` for one-off errors in non-library code.
- **Type Safety:** Leverage Rust's type system and SQLx's compile-time checked queries.
- **Configuration:** Use environment variables managed via `dotenvy` and the `Config` struct in `src/config.rs`.
- **Formatting:** Adhere to standard `rustfmt` guidelines.

## Implementation Details
- **`src/config.rs`**: Environment variable management (`Config` struct) using `dotenvy`.
- **`src/error.rs`**: Centralized `AppError` enum using `thiserror`. Implements `IntoResponse` for Axum and `is_unique_violation` for SQLx collision detection.
- **`src/models/url.rs`**: Contains `UrlModel` (DB), `UrlRequest` (DTO), and `UrlResponse` (DTO).
- **`src/repository/url_repo.rs`**: Database queries using SQLx (`create_url`, `get_url_by_code`, `increment_click_count`).
- **`src/services/`**:
    - `generator.rs`: Uses `nanoid!` for short codes.
    - `validator.rs`: Validates URLs (host, scheme, self-referencing check).
    - `url_service.rs`: Main business logic (orchestrates creation with retry logic and redirection with async click incrementing).
    - `cache.rs`: (Placeholder) Planned for Redis.
- **`src/handlers/`**:
    - `create.rs`: `create_url` endpoint, handles validation and IP logging.
    - `redirect.rs`: `redirect` endpoint, uses `resolve_url`.
    - `stats.rs`: (Placeholder) Planned for URL statistics.
- **`src/main.rs`**: Application entry point, router configuration (`POST /`, `GET /:code`), state initialization (`AppState`), and server setup.
- **`src/middleware/rate_limit.rs`**: (Placeholder) Planned for API rate limiting.

## Project Structure
```text
src/
├── config.rs       # Configuration loading
├── error.rs        # Error definitions
├── handlers/       # API endpoints
├── main.rs         # Entry point
├── middleware/     # Custom middleware
├── models/         # DTOs and Database models
├── repository/     # Persistence logic
├── services/       # Business logic
└── utils/          # Shared utilities
migrations/         # SQL schema migrations
```
