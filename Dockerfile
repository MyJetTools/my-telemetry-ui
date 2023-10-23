FROM ubuntu:22.04

COPY ./target/release/my-telemetry-ui ./target/release/my-telemetry-ui
COPY ./dist ./dist
ENTRYPOINT ["./target/release/my-telemetry-ui"]