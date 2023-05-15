# obsv

Super Simple Observability Platform

```sh
simple install script
```

## Overview

```mermaid
flowchart LR
    in_http["HTTP (Otlp, events)"]
    in_grpc[Otel GRPC]
    in_services[Services]

    out_api[API]
    out_dash[Dashboard]
    out_status[Status page]
    out_notifs[Notifications]

    subgraph obsv [Obsv]
        coll[Collector]
        monitor[Monitor]
        db["DB (ClickHouse)"]
        api[Data API]
        dash[Dashboard]
        status[Status page]
        notify[Notifier]
    end

    in_grpc --> |:4317| coll
    in_http --> |:4318| coll
    in_services --> monitor

    coll --> db
    db --> api
    monitor --> db
    db --> notify

    api --> |:5001| out_api
    dash --> |:5002| out_dash
    status --> |:5003| out_status
    notify --> |emails, hooks| out_notifs
```

## Repo organization

- `obsv-core`: core structures and utilities
- `obsv-collect`: collector service
- `obsv-monitor`: monitoring service
- `obsv-api`: service to serve data
- `obsv-notify`: notification service
- `obsv-dash`: dashboard server
- `obsv-status`: status page server
- `obsv-bin`: combined executables
- `lib/**`: shared libraries

## Similar tools

- [Jaeger](https://github.com/teletrace/teletrace): tracing collector and dashboard
- [Prometheus](https://prometheus.io/): metrics and alerting toolkit
- [teletrace](https://github.com/teletrace/teletrace): tracing collector and dashboard (ElasticSearch DB + Collector/Server).
- [Vigil](https://github.com/valeriansaliou/vigil): monitors, alerts, status page
- [QRYN](https://github.com/metrico/qryn): observability platform
- ...
