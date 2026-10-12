# NOTE
Biasakan menjalankan perintah dari root proyek, yaitu folder yang berisi Cargo.toml


# Dependency

| Dependency            | Tujuan                                            |
| --------------------- | ------------------------------------------------- |
| clap	                | Membuat CLI dengan subcommand dan validasi argumen|
| anyhow	            | Menangani error pada lapisan aplikasi             |
| tracing	            | Menghasilkan event log terstruktur                |
| tracing-subscriber	| Menampilkan dan memfilter log                     |

# Pending

untuk saat ini tokio, database driver, atau scheduler sekarang belum perlu diterapkan

# Flow

1. Periksa kompilasi
```bash
cargo check
```

2. Format kode
```bash
cargo fmt --all
```

3. Periksa kualitas kode
```bash
cargo clippy --all-targets -- -D warnings
```

4. Jalankan aplikasi
```bash
cargo run -- run
```

5. Coba bantuan CLI
```bash
cargo run -- --help
cargo run -- run --help
```

6. Aktifkan log debug
```powershell
$env:RUST_LOG = "debug"
cargo run -- run
```
> Untuk kembali ke pengaturan awal pada sesi terminal: `Remove-Item Env:RUST_LOG`

