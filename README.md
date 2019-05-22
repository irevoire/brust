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

## Dockerize

```
docker build -t brust . # Here brust si the name of the docker image
docker run -p 8787:8787 -v -v $PWD/config:/brust/config --name brust brust
```
