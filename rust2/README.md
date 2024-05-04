# Rust Axum - Web/API Development

## Watch Script 

This is a Watch Script for the Server. Everytime when we make any changes, the server will refresh and restart itself again.

`Main Script`

```cmd
cargo watch -q -c -w src/ -x run
```

`Test Script`

```cmd
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```