# NEARS Template - README ES

NEARS es un framework SSR en Rust inspirado en la ergonomía de Next.js, pero implementado con el stack real de Rust: Axum, Maud, PostgreSQL, HTMX, Alpine y Tailwind CLI. La app renderiza HTML en servidor, usa rutas por archivos, separa `page.rs` y `route.rs`, y mantiene autenticación y acceso a datos en servidor.

## Stack

- `axum` + `tokio`
- `maud`
- `sqlx` + PostgreSQL
- `htmx`
- `alpine.js`
- Tailwind CSS v4 CLI
- JWT + `CookieJar` + `bcrypt`

## Arquitectura base

```text
nears/
├── app/            # Rutas visuales y handlers HTTP
├── components/     # Componentes globales
├── lib/            # Auth, DB, DAL y types
├── migrations/     # SQL de esquema
├── public/         # Assets estáticos
├── scripts/        # Seed y utilidades
├── build.rs        # Generador del router
├── main.rs         # Servidor Axum
├── Makefile
└── nears           # CLI local
```

## Principios

1. HTML renderizado en servidor.
2. Enrutado por sistema de archivos autogenerado por `build.rs`.
3. `page.rs` para vistas y `route.rs` para handlers HTTP.
4. Auth tipo DAL: guards en servidor, no en cliente.
5. `HX-Trigger` sin redirect completo y cookie efímera solo cuando hay redirect completo.
6. Respuestas sensibles con `no-store`.

## Requisitos

Antes de correr el proyecto en tu computadora necesitas:

1. Rust estable instalado
2. Cargo
3. PostgreSQL corriendo localmente
4. Tailwind CSS Standalone CLI
5. Linux para `tailwindcss-linux-x64`, o el binario standalone equivalente para tu sistema operativo

## Instalación local

### 1. Clonar el repositorio

```bash
git clone https://github.com/jisidrojv/nears.git
cd nears
```

### 2. Descargar Tailwind CSS Standalone CLI

En Linux:

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
```

Si usas macOS o Windows, descarga el binario standalone equivalente desde los releases de Tailwind CSS y colócalo en la raíz del proyecto con el nombre `tailwindcss-linux-x64`, o ajusta la variable `TAILWIND` del `Makefile`.

### 3. Crear la base de datos

Crea una base en PostgreSQL, por ejemplo:

```sql
CREATE DATABASE nears;
```

### 4. Configurar `.env`

Crea un archivo `.env` dentro de `nears/`:

```env
HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgresql://usuario:password@localhost:5432/nears
JWT_SECRET=cambia-esto-por-un-secreto-largo
```

También puedes partir de:

```bash
cp .env.example .env
```

### 5. Resetear esquema y sembrar datos iniciales

```bash
./nears reset
```

Ese comando:

1. recrea el esquema
2. corre migraciones
3. ejecuta `scripts/seed.rs`

## Ejecutar en desarrollo

```bash
./nears dev
```

Eso compila CSS y arranca el servidor.

Abre:

```text
http://127.0.0.1:3000
```

## Credenciales iniciales

Después del seed, la app crea este usuario:

- email: `admin@nears.dev`
- password: `nears/admin!`

## Comandos útiles

- `./nears dev` -> compila CSS y levanta el servidor
- `./nears reset` -> recrea DB y ejecuta seed
- `./nears build` -> build de producción
- `./nears clean` -> limpia artefactos

## Flujo funcional actual

La demo incluida permite:

1. acceder con sesión
2. proteger `/dashboard`
3. crear registros
4. editar registros
5. eliminar registros
6. mostrar toasts
7. cambiar tema claro/oscuro

## Patrón de autenticación

- páginas privadas: `require_session(&jar)`
- endpoints HTMX privados: `require_htmx_session(&jar)`
- respuestas sensibles: `no_store_response(...)`
- nunca `Set-Cookie` manual para sesión o toast

## Build de producción

```bash
./nears build
```

El binario queda en:

```text
target/release/nears-template
```

Debes copiar también:

1. `public/`
2. `.env`

## Nota

Este repositorio mantiene `README_ES.md` y `README_EN.md` por separado.
