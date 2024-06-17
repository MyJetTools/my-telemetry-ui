FROM ubuntu:22.04
COPY ./target/release/my-telemetry-ui ./target/release/my-telemetry-ui
COPY ./dist /target/release/dist
RUN chmod +x /target/release/my-telemetry-ui
WORKDIR /target/release/
ENTRYPOINT ["./target/release/my-telemetry-ui"]