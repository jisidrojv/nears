![NEARS Logo](nears-logo.png)

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
4. Tailwind CSS standalone CLI
5. Linux for `tailwindcss-linux-x64`, or the matching Tailwind standalone binary for your OS

### 1. Clone the repository

```bash
git clone https://github.com/jisidrojv/nears.git
cd nears
```

### 2. Download Tailwind CSS standalone CLI

Linux:

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
```

If you are on macOS or Windows, download the matching standalone binary from the Tailwind CSS releases page and place it in the project root as `tailwindcss-linux-x64`, or adjust the `TAILWIND` variable in `Makefile`.

### 3. Create a PostgreSQL database

```sql
CREATE DATABASE nears;
```

### 4. Configure `.env`

Create `nears/.env`:

```env
HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgresql://user:password@localhost:5432/nears
JWT_SECRET=replace-this-with-a-long-random-secret
```

You can also start from:

```bash
cp .env.example .env
```

### 5. Reset schema and seed data

```bash
./nears reset
```

### 6. Run the app

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
