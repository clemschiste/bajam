start:
    export SQLX_OFFLINE=true && cd bajam-daemon/ && cargo run --bin setup && cargo run --bin bajam-daemon

test:
    export SQLX_OFFLINE=true && cargo test

prep:
    cargo sqlx prepare --workspace --database-url sqlite://./bajam-daemon/data/app.db
