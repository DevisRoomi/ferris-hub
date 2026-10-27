# FerrisHub - Rust Axum & Vue Neo-Grotesk Bento Grid

Fullstack modern dashboard arsitektur **Rust Axum 0.7** & **Vue 3** bertema **Neo-Grotesk Bento Grid**, dirancang untuk developer ecosystem pemula & profesional.

## Fitur Utama
- **Backend Rust (Axum 0.7)**: High performance async web server, multi-threaded Tokio runtime.
- **Security & JWT**: Otentikasi stateless dengan JSON Web Token (HS256) claim parsing.
- **Frontend Vue 3 Reactivity**: Reactive state management ala TanStack Query pattern.
- **Neo-Grotesk Bento Grid UI**: Typography Space Grotesk + JetBrains Mono, modular auto-flow cards.
- **Integrated Tooling**:
  - **Laravel Herd**: Langsung muncul di list sites Herd via `ferris-hub.test`.
  - **FlyEnv MySQL Ready**: Siap dikoneksikan ke MySQL pool port 3306.

## Struktur Project
```
ferris-hub/
├── backend/            # Rust Axum REST API Server (Port 8088)
│   ├── src/main.rs     # Endpoint auth, projects bento grid, audit event logs
│   └── Cargo.toml
├── public/             # Frontend Vue 3 Bento Grid Web Interface
│   └── index.html      # Tailwind CSS + Space Grotesk + Vue 3 App
└── README.md
```

## Menjalankan Project

### 1. Backend Rust
```bash
cd backend
cargo run --release
```
API berjalan di: `http://127.0.0.1:8088`

### 2. Frontend & Laravel Herd
Karena folder berada langsung di direktori Herd (`C:\Users\Lenovo\Herd\ferris-hub`), site otomatis terdeteksi:
- Buka browser: `http://ferris-hub.test`
- Atau via Herd GUI: klik site **ferris-hub**

### 3. Akun Default JWT Demo
- **Username**: `admin`
- **Password**: `ferris123`
