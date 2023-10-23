#[my_grpc_extensions::client::generate_grpc_client(
    proto_file: "./proto/TelemetryReader.proto",
    crate_ns: "crate::reader_grpc",
    retries: 3,
    request_timeout_sec: 20,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct TelemetryReaderGrpcClient;
