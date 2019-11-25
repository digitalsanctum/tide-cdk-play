
# tide-play

Minimal example for using the [Tide](https://docs.rs/tide/0.3.0/tide/) framework.

## build

```bash
cargo build --release
```

## run

```bash
cd target/release
./tide-play
```

## usage

Create a message:
```bash
curl -X POST \
  http://localhost:8000/message \  
  -H 'Content-Type: application/json' \
  -d '{ "contents": "foo", "author": "shane" }'
```

Get a message:
```bash
curl -X GET http://localhost:8000/message/0  
```
