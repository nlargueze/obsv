# obsv-tester

Tools to send test traces/logs/metrics

## Proto definitions

Proto definitions for the OpenTelemetry collector have been downloaded from `https://github.com/open-telemetry/opentelemetry-proto`.

## Dev notes

- `https://ghz.sh/` is a load testing tool for gRPC (tried but fiddly as per quick tests)
- the error `http2 error: connection error detected: frame with invalid size` with hyper seems to be because the server does not have ALPN negotiation.
- with HTTP/1.1, https is HTTP over TLS
- TCP handshake: SYN -> SYN-ACK -> SYN ==> client sends ClientHello => server sends ServerHello
- A solid article on HTTP, TLS, rust setup: https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for
- https://blog.logrocket.com/best-rust-http-client/
- https://www.digitalocean.com/community/tutorials/http-1-1-vs-http-2-what-s-the-difference
- HTTP2 negotiation: https://matthewparrilla.com/post/negotiation-http2-alpn-tls-handshake/
- CORS: https://medium.com/@baphemot/understanding-cors-18ad6b478e2b

### gRPC
