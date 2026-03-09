# NEARS

NEARS is an SSR web framework in Rust inspired by the ergonomics of Next.js, but built with a native Rust stack: Axum, Maud, PostgreSQL, HTMX, Alpine, and Tailwind CLI.

It focuses on:

1. file-based routing
2. server-rendered HTML
3. DAL-style authentication
4. HTMX-driven mutations
5. single-binary deployment

## Demo Video

[![Watch the demo video](https://img.youtube.com/vi/wW91aSq9GPs/hqdefault.jpg)](https://youtu.be/wW91aSq9GPs)

Watch the project demo here:

- https://youtu.be/wW91aSq9GPs

## Quick Start

### Requirements

1. Rust stable
2. Cargo
3. PostgreSQL running locally
4. Linux or a compatible environment for `tailwindcss-linux-x64`

### 1. Clone the repository

```bash
git clone <your-repo>
cd <your-repo>/nears
```

### 2. Create a PostgreSQL database

```sql
CREATE DATABASE nears;
```

### 3. Configure `.env`

Create `nears/.env`:

```env
HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgresql://user:password@localhost:5432/nears
JWT_SECRET=replace-this-with-a-long-random-secret
```

### 4. Reset schema and seed data

```bash
./nears reset
```

### 5. Run the app

```bash
./nears dev
```

Open:

```text
http://127.0.0.1:3000
```

Default seeded credentials:

- email: `admin@nears.dev`
- password: `nears/admin!`

## Main Commands

- `./nears dev` -> compile CSS and run the app
- `./nears reset` -> recreate schema and seed data
- `./nears build` -> production build
- `./nears clean` -> clean build artifacts

## Documentation

- English: [README_EN.md](README_EN.md)
- Español: [README_ES.md](README_ES.md)

## Demo Features

The current demo includes:

1. login with session auth
2. private dashboard protection
3. record creation
4. record editing
5. record deletion
6. toast notifications
7. light and dark theme toggle
