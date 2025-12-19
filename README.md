**Beware:** Curiosity Driven Development

# reqwest-client
A minimal [DummyJSON](https://dummyjson.com/docs) [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html) client.

Logging/Tracing:
- HEAD uses the [tracing](https://docs.rs/tracing/latest/tracing/) framework
- HEAD~ uses the [log](https://docs.rs/log/latest/log/) + [env_logger](https://docs.rs/env_logger/latest/env_logger/) crates

### Usage

```
igor@gioco:~/git/reqwest-client$ RUST_LOG=reqwest=trace cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/reqwest-client`
2025-12-19T20:20:49.274465Z  INFO span: reqwest_client: Before API call
2025-12-19T20:20:49.335680Z DEBUG span: reqwest::connect: starting new connection: https://dummyjson.com/
2025-12-19T20:20:49.385406Z TRACE reqwest::connect::verbose: 274d2d2d write: b"GET /test HTTP/1.1\r\naccept: */*\r\nhost: dummyjson.com\r\n\r\n"
2025-12-19T20:20:49.707463Z TRACE reqwest::connect::verbose: 274d2d2d read: b"HTTP/1.1 200 OK\r\nDate: Fri, 19 Dec 2025 20:20:49 GMT\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: 30\r\nConnection: keep-alive\r\nServer: cloudflare\r\nx-ratelimit-limit: 100\r\nx-ratelimit-remaining: 99\r\nx-ratelimit-reset: 1766175651\r\nx-dns-prefetch-control: off\r\nx-frame-options: SAMEORIGIN\r\nstrict-transport-security: max-age=15552000; includeSubDomains\r\nx-download-options: noopen\r\nx-content-type-options: nosniff\r\nx-xss-protection: 1; mode=block\r\naccess-control-allow-origin: *\r\netag: W/\"1e-X/ZTgL0+qpgHuGmBISFBqxNg62E\"\r\nvary: Accept-Encoding\r\ncf-cache-status: DYNAMIC\r\nNel: {\"report_to\":\"cf-nel\",\"success_fraction\":0.0,\"max_age\":604800}\r\nReport-To: {\"group\":\"cf-nel\",\"max_age\":604800,\"endpoints\":[{\"url\":\"https://a.nel.cloudflare.com/report/v4?s=YTlOr...W4%3D\"}]}\r\nCF-RAY: 9b099a50beff27b8-VIE\r\n\r\n{\"status\":\"ok\",\"method\":\"GET\"}"
2025-12-19T20:20:49.708900Z TRACE span: reqwest::retry: shouldn't retry!
Response: Object {"method": String("GET"), "status": String("ok")}
2025-12-19T20:20:49.710024Z  INFO span: reqwest_client: After API call
```
