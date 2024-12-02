# TemanCurhat
# Aplikasi Teman Curhat

Aplikasi ini memungkinkan pengguna untuk curhat dengan orang acak. Dibangun dengan menggunakan Rust dan jaringan ICP (Internet Computer Protocol).

## Spesifikasi Aplikasi

- Pengguna dapat mendaftar dengan nama mereka.
- Pengguna dapat mengirim pesan dan menerima pesan dari teman curhat yang acak.
- Backend menggunakan Rust dan ICP untuk menangani logika aplikasi dan komunikasi antar pengguna.
- Frontend dibangun dengan Rust dan WebAssembly menggunakan Yew untuk antarmuka pengguna.

## Cara Menjalankan

### Backend
1. Masuk ke direktori `backend/`.
2. Bangun dan jalankan backend:
    ```bash
    cargo build --release
    cargo run
    ```

### Frontend
1. Masuk ke direktori `frontend/`.
2. Bangun frontend dengan WebAssembly:
    ```bash
    wasm-pack build --target web
    ```
3. Jalankan server lokal untuk mengakses aplikasi web:
    ```bash
    python3 -m http.server 8080
    ```

Akses aplikasi di `http://localhost:8080`.
