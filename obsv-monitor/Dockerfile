FROM rust:1.69.0
WORKDIR /usr/src/myapp
COPY . .
COPY ./monitor.cfg.toml /etc/monitor/config.toml
RUN cargo install --path .
CMD ["monitor", "--cfg", "/etc/monitor/config.toml"]