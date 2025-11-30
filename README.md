# yuan

> A common utils lib.

## yuan-post-json
> yuan-post-json path/to/request_file

struct of request_file:
```txt
https://httpbin.org/post

{
  "name": "test",
  "value": 123,
  "nested": {
    "key": "value"
  }
}
```
The first line is the URL, the second line is a blank line and the rest is the JSON body.

The response will be recorded as `yuna-post-json-xxxx` in the current directory.

Sample response:
```txt
> POST https://httpbin.org/post HTTP/1.1
> Host: httpbin.org
> content-type: application/json
>
{
  "name": "test",
  "value": 123,
  "nested": {
    "key": "value"
  }
}

< HTTP/1.1 200 OK
< date: Sun, 30 Nov 2025 10:53:05 GMT
< content-type: application/json
< content-length: 531
< connection: keep-alive
< server: gunicorn/19.9.0
< access-control-allow-origin: *
< access-control-allow-credentials: true
<
{
  "args": {},
  "data": "{\n  \"name\": \"test\",\n  \"value\": 123,\n  \"nested\": {\n    \"key\": \"value\"\n  }\n}",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "74",
    "Content-Type": "application/json",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-692c220f-3e86d99a42fbf3417cf8b0c8"
  },
  "json": {
    "name": "test",
    "nested": {
      "key": "value"
    },
    "value": 123
  },
  "origin": "38.175.103.97",
  "url": "https://httpbin.org/post"
}
```