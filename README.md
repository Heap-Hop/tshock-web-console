# Tshock-Web-Console
A Terraria servers web console by using tshock api

### Run Web Server
Written in Rust

```
# server will listen at 8488 by default
cargo run

# or build
cargo build --release
./target/release/tshock-web-console

# Config
# edit the Settings.md or use environment with a prefix 'TR' like:
TR_PORT=8888 ./target/release/tshock-web-console

```

### Dev Web UI
Written in TS with Vue3 + Naive UI
```
cd webui

# dev
yarn serve

# build
yarn build

# you may change your devServer in vue.config.js
```
