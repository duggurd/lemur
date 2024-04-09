wsl -d ubuntu -- "~/.cargo/bin/cargo" build --target x86_64-unknown-linux-gnu --release --bins
cargo build --target x86_64-pc-windows-msvc --release --bins