# Harmony

Harmony is a work-in-progress personal media server that aims to support (in the future) all types of media, such as music, books, movies, shows, podcasts, etc. Currently, it scans music libraries for files, extracts metadata, stores it in a SQLite database, and serves it via REST endpoints.

## Building

First, export environment variables with:

``` shell
source .env.local
export HARMONY_KEY
```

Then, run the cargo build process:

``` shell
cargo build               # Debug build
cargo build --release     # Release build
cargo run                 # Run the server
```
