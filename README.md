# quick_link
A very short and very sweet link shortener that operates from an LRU cache, when the cache is full it will begin discarding the least recently used cache elements. This is helpful for applications where you do not need shortlinks to live forever, but do need popular ones to function for as long as possible. 

## Configuration
There's a Rocket.toml file which you can configure to change the IP and port to bind to, the default cache size is 10,000.

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
