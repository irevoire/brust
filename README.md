# Brüst

My discord bot in rust

## Compiling

```
cargo build --release
```

## Running

```
cargo run --release
```

## Docker

### Build
```
docker build .
```

### Run
```
docker run --restart=always -d -v $PWD/config:/brust/config --name brust -e DISCORD_TOKEN=YOUR_DISCORD_TOKEN brust
```
