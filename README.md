# Harmony

Harmony is a work-in-progress personal media server that aims to support (in the future) all types of media, such as music, books, movies, shows, podcasts, etc. Currently, it scans music and book libraries for files, extracts metadata, stores it in a SQLite database, and serves it via REST endpoints.

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

## Roadmap

- [x] Storage and retrieval support for music
- [x] Storage and retrieval support for books
- [ ] Storage and retrieval support for podcasts
- [ ] Storage and retrieval support for shows
- [ ] Storage and retrieval support for movies
- [ ] Support various file types
  - [x] FLAC
  - [ ] MP3
  - [ ] WAV
  - [x] EPUB
  - [ ] PDF
- [ ] Full metadata support for music
  - [ ] Full album data
  - [ ] Full track data
  - [ ] Disc data
  - [ ] Artist data
  - [ ] Lyric storage and retrieval
  - [ ] Genre data
  - [ ] Track playback/interaction analytics
- [ ] Full metadata support for books
- [ ] Full metadata support for podcasts
- [ ] Full metadata support for shows
- [ ] Full metadata support for movies
- [ ] Full shelf support
  - [x] Playlist support
  - [x] Starring support
  - [ ] Booklist support
  - [ ] Custom analytics
- [ ] OpenSubsonic compatability layer
