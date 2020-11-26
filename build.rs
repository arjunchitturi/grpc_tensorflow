fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("/Users/arjun/work/rust/grpc_tensorflow/tensorflow") // destination for output.
        .compile(&["pb/prediction_service.proto"], &["pb"]) // dump path of tensorflow proto files.
        .expect("failed to compile tensorflow protos");
}
