# Frontend API Reference

Complete backend reference for frontend implementation.

**Base URL:** `http://localhost:8080` (configurable via `APP_DOMAIN`)

---

## Error Format

All errors follow this structure:

```json
{
  "error": "Description of the error"
}
```

---

## Rate Limiting

All endpoints are rate-limited to **10 requests per second** with a **burst of 30**.
When exceeded, the server returns `429 Too Many Requests`.

---

## Authentication

The backend uses **PASETO v4** tokens (similar to JWT but more secure). Tokens expire after **24 hours**.

### How to send the token

For protected endpoints, include the token in the `Authorization` header:

```
Authorization: Bearer <token>
```

### Protected endpoints

| Endpoint | Requires Auth |
|---|---|
| `POST /` | No |
| `GET /{code}` | No |
| `GET /stats/{code}` | **Yes** |
| `POST /auth/register` | No |
| `POST /auth/login` | No |
| `POST /auth/google` | No |

---

## Endpoints

### 1. Register

`POST /auth/register`

#### Request Body

```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "mypassword123"
}
```

#### Validations (apply these on the frontend too)

| Field | Rule | Error Message |
|---|---|---|
| `username` | Min 3 characters, Max 50 characters | `Username must be between 3 and 50 characters` |
| `email` | Must be a valid email format | `Invalid email address` |
| `password` | Min 8 characters | `Password must be at least 8 characters` |

#### Success Response — `201 Created`

```json
{
  "token": "v4.local.xxxxxx...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "john@example.com",
    "username": "johndoe"
  }
}
```

#### Error Responses

| Status | Error | When |
|---|---|---|
| `400` | Validation error message | Invalid input (see validations above) |
| `409` | `Username or email already exists` | Email or username is taken |

---

### 2. Login

`POST /auth/login`

#### Request Body

```json
{
  "email": "john@example.com",
  "password": "mypassword123"
}
```

#### Validations (apply these on the frontend too)

| Field | Rule | Error Message |
|---|---|---|
| `email` | Must be a valid email format | `Invalid email address` |
| `password` | Must not be empty | `Password is required` |

#### Success Response — `200 OK`

```json
{
  "token": "v4.local.xxxxxx...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "john@example.com",
    "username": "johndoe"
  }
}
```

#### Error Responses

| Status | Error | When |
|---|---|---|
| `400` | Validation error message | Invalid input |
| `401` | `Invalid credentials` | Wrong email or password |
| `401` | `This account requires external authentication. Please sign in using your social provider.` | User registered via Google, has no password |

---

### 3. Google OAuth

`POST /auth/google`

#### Flow

1. Frontend redirects user to Google OAuth consent screen
2. Google redirects back with an authorization `code`
3. Frontend sends that `code` to this endpoint
4. Backend exchanges the code for an access token with Google, fetches user info, and creates/links the account

#### Request Body

```json
{
  "code": "4/0AX4XfWh..."
}
```

#### Success Response — `200 OK`

```json
{
  "token": "v4.local.xxxxxx...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "john@gmail.com",
    "username": "John Doe"
  }
}
```

#### Behavior

- If the Google account **already exists** in the database (by `google_id`): logs in directly.
- If the **email already exists** but without Google linked: links the Google account to the existing user.
- If the user is **completely new**: creates a new account. If the username (from Google) is already taken, a random 4-character suffix is appended (e.g., `John Doe_a1b2`).

#### Error Responses

| Status | Error | When |
|---|---|---|
| `400` | `Google authentication failed: {status}` | Invalid or expired authorization code |
| `400` | `Failed to retrieve Google user info: {status}` | Problem fetching user profile from Google |

---

### 4. Create Short URL

`POST /`

#### Request Body

```json
{
  "original_url": "https://www.example.com/very/long/path"
}
```

#### Validations (apply these on the frontend too)

| Rule | Error Message |
|---|---|
| Must be a valid URL | `Invalid URL format` |
| Only `http` and `https` protocols | `Only http and https protocols are allowed` |
| Cannot be `localhost`, `127.0.0.1`, or `::1` | `Cannot shorten URLs pointing to localhost` |
| Cannot be a private IP (10.x, 192.168.x, etc.) | `Cannot shorten URLs pointing to private IP addresses` |
| Must have a domain + TLD (e.g., `.com`) | `URL must include a domain and a TLD (e.g., .com)` |
| TLD must be at least 2 characters | `The TLD is too short to be a valid public domain` |
| Cannot be the app's own domain | `Cannot shorten URLs from this domain` |
| Must pass Google Safe Browsing check | `URL malicious` |

#### Success Response — `201 Created`

```json
{
  "short_code": "aB3xK9mZ",
  "original_url": "https://www.example.com/very/long/path",
  "short_url": "http://localhost:8080/aB3xK9mZ",
  "created_at": "2026-03-02T12:00:00Z",
  "expires_at": "2026-04-01T12:00:00Z"
}
```

#### Important Notes

- Short codes are **8 characters** long, alphanumeric (a-z, A-Z, 0-9).
- URLs **expire after 30 days** from creation.
- The `short_url` field contains the full URL ready to share.
- The `expires_at` field is always present (ISO 8601 format).

#### Error Responses

| Status | Error | When |
|---|---|---|
| `400` | Validation error message | Invalid URL (see rules above) |
| `403` | `URL malicious` | URL flagged by Google Safe Browsing |
| `409` | `Resource already exists` | Short code collision (rare, retried 4 times internally) |

---

### 5. Redirect

`GET /{code}`

Redirects to the original URL.

- Returns a **301 Permanent Redirect**.
- If the URL has expired, returns `410 Gone`.
- If the code doesn't exist, returns `404 Not Found`.
- Analytics (click count, user agent, IP, referrer) are recorded asynchronously in the background.

#### Error Responses

| Status | Error | When |
|---|---|---|
| `404` | `URL not found` | Short code does not exist |
| `410` | `The requested URL has expired and is no longer available` | URL has passed its expiration date |

---

### 6. Get URL Stats (Protected)

`GET /stats/{code}`

**Requires authentication** — send the token in the `Authorization` header.

#### Success Response — `200 OK`

```json
{
  "short_code": "aB3xK9mZ",
  "short_url": "http://localhost:8080/aB3xK9mZ",
  "original_url": "https://www.example.com/very/long/path",
  "total_clicks": 42,
  "created_at": "2026-03-02T12:00:00Z",
  "expires_at": "2026-04-01T12:00:00Z",
  "stats": [
    {
      "id": 1,
      "url_id": 10,
      "user_agent": "Mozilla/5.0 ...",
      "ip_address": "203.0.113.50/32",
      "referrer": "https://twitter.com",
      "country_code": null,
      "created_at": "2026-03-02T14:30:00Z"
    }
  ]
}
```

#### Notes

- `stats` is an array of every individual click, ordered by most recent first.
- `user_agent`, `ip_address`, `referrer`, and `country_code` can be `null`.
- `total_clicks` is the aggregate count.

#### Error Responses

| Status | Error | When |
|---|---|---|
| `401` | (empty body) | Missing or invalid token |
| `404` | `URL not found` | Short code does not exist |

---

## Frontend Validation Summary

Here are all the validations you should implement on the frontend to match the backend:

### Register Form

```
username:  required, min 3 chars, max 50 chars
email:     required, valid email format
password:  required, min 8 chars
```

### Login Form

```
email:     required, valid email format
password:  required, not empty
```

### Shorten URL Form

```
url:       required, valid URL format, must start with http:// or https://
```

---

## Token Storage

- Store the token from login/register responses (e.g., `localStorage` or a cookie).
- Include it in the `Authorization: Bearer <token>` header for protected routes.
- Tokens expire after **24 hours**. When a `401` is received, redirect the user to login.

---

## CORS

The backend allows requests from the domain configured in `APP_DOMAIN`.
Allowed methods: `GET`, `POST`, `PATCH`, `DELETE`.
Allowed headers: `Content-Type`, `Authorization`.

If the frontend runs on a different origin, update `APP_DOMAIN` in the backend's `.env`.

---

## Google OAuth Setup (Frontend Side)

1. Redirect the user to:

```
https://accounts.google.com/o/oauth2/v2/auth?
  client_id=<GOOGLE_CLIENT_ID>&
  redirect_uri=<GOOGLE_REDIRECT_URI>&
  response_type=code&
  scope=openid email profile&
  access_type=offline
```

2. Google redirects back to `GOOGLE_REDIRECT_URI` with a `?code=...` query parameter.

3. Extract the `code` and send it to `POST /auth/google`:

```json
{
  "code": "4/0AX4XfWh..."
}
```

4. The backend handles everything else (token exchange, user creation/linking).

---

## Common HTTP Status Codes

| Code | Meaning |
|---|---|
| `200` | Success |
| `201` | Created (register, shorten URL) |
| `301` | Permanent redirect (short URL -> original URL) |
| `400` | Bad request / validation error |
| `401` | Unauthorized (missing/invalid/expired token, wrong credentials) |
| `403` | Forbidden (malicious URL) |
| `404` | Not found |
| `409` | Conflict (duplicate resource) |
| `410` | Gone (expired URL) |
| `429` | Too many requests (rate limited) |
| `500` | Internal server error |
