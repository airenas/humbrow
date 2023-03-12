[![Rust](https://github.com/airenas/humbrow/actions/workflows/rust.yml/badge.svg)](https://github.com/airenas/humbrow/actions/workflows/rust.yml)[![Snyk vulnerabilities Tests](https://github.com/airenas/humbrow/actions/workflows/snyk.yml/badge.svg)](https://github.com/airenas/humbrow/actions/workflows/snyk.yml)

# HUMBROW

Docker image to bypass Cloudflare checks. Works as HUMan BROWser.

Based on https://github.com/ultrafunkamsterdam/undetected-chromedriver. Packed as WS to retrieve cloudflare cookie using API. The goal is to retrieve `cf_clearance` cookie and forget about `undetected-chromedriver`. 

## Usage
## Run docker container

```bash
docker run -it -p 8000:8000 airenas/humbrow:0.1.14-c784977
```

### Get cookie
```bash
curl localhost:8000/cookie?url=ignitis.lt
```
```json
{"agent":"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
"cookie":"yQQHB2PuMuYUi7F2DnJVqIVLPkYhtcBUF0HN_dJUGd8-1678615299-0-150"}
```
### Use

Use in your favorite web crawler pipeline passing the returned `user-agent` and setting `cf_clearance` cookie.

