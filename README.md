# quick_link
A very short and very sweet link shortener that operates from an LRU cache.


## Saving link
```
curl --location 'http://127.0.0.1:8000/' \
--header 'Content-Type: text/plain' \
--data 'https://google.com'
```
The above will return a short code

## Accessing Link
http://127.0.0.1:8000/<code>

