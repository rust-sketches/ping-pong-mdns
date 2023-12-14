# ping-pong-mdns

## to run locally

In one terminal, run `ping`

```
cargo run --bin ping
```

In a second terminal, run `pong`

```
cargo run --bin pong
```

In a third terminal, kick off the process by sending a message to one of them

```
curl -v -X POST 127.0.0.1:7878/pong
```

Watch them play `ping` / `pong` in the first two terminals.

## to run in Docker containers

In one terminal, build the `ping` image, then run the container

```
docker build -t ping -f ping.Dockerfile . && docker run --name ping -p 7878:7878 ping
```

In a second terminal, build the `pong` image, then run the container

```
docker build -t pong -f pong.Dockerfile . && docker run --name pong -p 8787:8787 pong
```

In a third terminal, kick off the process by sending a message to one of them

```
curl -v -X POST 127.0.0.1:8787/pong
```