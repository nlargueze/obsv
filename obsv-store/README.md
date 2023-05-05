# obsv-db

Database management for obsv

## Evaluations

- `Clickhouse` seems to be a solid OLAP db to store logs and analytics

## Dev notes

Deploy

```sh
# Homebrew
brew install clickhouse

# Docker
docker run --ulimit nofile=262144:262144 clickhouse/clickhouse-server
```
