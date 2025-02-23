fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("/Users/arjun/work/rust/grpc_tensorflow/tensorflow") // destination for output.
        .compile_protos(
            &[
                "pb/input.proto",
                "pb/op_def.proto",
                "pb/node_def.proto",
                "pb/graph.proto",
                "pb/regression.proto",
                "pb/tensor_shape.proto",
                "pb/predict.proto",
                "pb/function.proto",
                "pb/variable.proto",
                "pb/prediction_service.proto",
                "pb/struct.proto",
                "pb/get_model_metadata.proto",
                "pb/types.proto",
                "pb/meta_graph.proto",
                "pb/versions.proto",
                "pb/attr_value.proto",
                "pb/inference.proto",
                "pb/saver.proto",
                "pb/model.proto",
                "pb/tensor.proto",
                "pb/feature.proto",
                "pb/resource_handle.proto",
                "pb/trackable_object_graph.proto",
                "pb/saved_object_graph.proto",
                "pb/example.proto",
                "pb/classification.proto",
            ],
            &["pb"],
        )
        .expect("failed to compile tensorflow protos");
}
