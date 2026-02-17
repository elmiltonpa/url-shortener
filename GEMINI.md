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
- [x] **Data Models**: Define `UrlModel`, `UrlRequest`, and `UrlResponse`.
- [x] **Error Handling**: Professional error system with `thiserror` and Axum integration.
- [ ] **Repository Layer**: Implementation of database interactions (PostgreSQL).
- [ ] **Business Logic (Services)**:
    - [ ] Short code generation.
    - [ ] Redis caching.
    - [ ] URL validation.
- [ ] **API Handlers**:
    - [ ] `POST /`: Create shortened URL.
    - [ ] `GET /:code`: Redirect to original URL.
    - [ ] `GET /stats/:code`: View URL statistics.
- [ ] **Server Setup**: Wire everything together in `main.rs`.

## Development Conventions
- **Asynchronous First:** All I/O operations should be `async` using `tokio`.
- **Error Handling:** Use the custom `AppError` type in `src/error.rs`. Prefer `anyhow` for one-off errors in non-library code.
- **Type Safety:** Leverage Rust's type system and SQLx's compile-time checked queries.
- **Configuration:** Use environment variables managed via `dotenvy` and the `Config` struct in `src/config.rs`.
- **Formatting:** Adhere to standard `rustfmt` guidelines.

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
