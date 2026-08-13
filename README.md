# quick_link
A very short and very sweet link shortener that operates from an LRU cache, when the cache is full it will begin discarding the least recently used cache elements. This is helpful for applications where you do not need shortlinks to live forever, but do need popular ones to function for as long as possible. 

## Configuration
There's a Rocket.toml file which you can configure to change the IP and port to bind to, and `cache_size` to change how many links are held before the least recently used ones are discarded. The default cache size is 10,000.

Every setting can also be given as an environment variable — `ROCKET_ADDRESS`, `ROCKET_PORT`, `ROCKET_CACHE_SIZE` — which take precedence over Rocket.toml.

## Docker
```
docker compose up --build
```
The service listens on `http://127.0.0.1:8000` on the host.

Or without compose:
```
docker build -t quick_link .
docker run --rm -p 8000:8000 quick_link
```

Rocket.toml is not baked into the image, since its `127.0.0.1` bind address is
unreachable from outside a container. The image defaults to `ROCKET_ADDRESS=0.0.0.0`
and `ROCKET_PORT=8000`; override those (or `ROCKET_CACHE_SIZE`) with environment
variables:
```
docker run --rm -e ROCKET_PORT=9000 -e ROCKET_CACHE_SIZE=50000 -p 9000:9000 quick_link
```

## Saving link
```
curl --location 'http://127.0.0.1:8000/' \
--header 'Content-Type: text/plain' \
--data 'https://google.com'
```
The above will return a short code

## Accessing Link
```
http://127.0.0.1:8000/<code>
```
