use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use tensorflowlib::{
    model_spec, prediction_service_client::PredictionServiceClient, tensor_shape_proto::Dim,
    ModelSpec, PredictRequest, PredictResponse, TensorProto, TensorShapeProto,
};

fn prepare_request(
    shape: (i64, i64, i64),
    service_name: &str,
    signature_name: &str,
    version: i64,
) -> tonic::Request<PredictRequest> {
    let mut inputs = HashMap::<String, TensorProto>::new();
    let mut all_tensors: Vec<u8> = vec![];

    // fetch some random floats in the range 0..10.
    let step = Uniform::new(0.0, 10.0);
    let input_vec: Vec<f32> = step
        .sample_iter(&mut rand::thread_rng())
        .take(shape.0 as usize)
        .collect();

    println!("INPUTS: {:#?}", input_vec);
    for f in input_vec {
        all_tensors.extend_from_slice(&f.to_ne_bytes());
    }
    let tensor = TensorProto {
        dtype: 1i32,
        tensor_shape: Some(TensorShapeProto {
            dim: vec![
                Dim {
                    size: shape.0,
                    name: "rows".to_string(),
                },
                Dim {
                    size: shape.1,
                    name: "dim1".to_string(),
                },
                Dim {
                    size: shape.2,
                    name: "dim2".to_string(),
                },
            ],
            unknown_rank: false,
        }),
        version_number: 0i32,
        tensor_content: all_tensors,
        float_val: vec![0f32],
    };
    inputs.insert("x".to_string(), tensor);
    tonic::Request::new(PredictRequest {
        model_spec: Some(ModelSpec {
            name: service_name.to_string(),
            signature_name: signature_name.to_string(),
            version_choice: Some(model_spec::VersionChoice::Version(version)),
        }),
        inputs: inputs,
        output_filter: vec!["y".to_string()],
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_shape = (3, 1, 1); //3 x (1, 1)
    let (service_name, signature_name, version) = ("half_plus_two", "serving_default", 123);
    let request: tonic::Request<PredictRequest> =
        prepare_request(input_shape, service_name, signature_name, version);
    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:8500")
        .connect()
        .await?;
    let mut client = PredictionServiceClient::new(channel);
    println!("Calling Half Plus Two Service (y = x*0.5 + 2):");
    let response: PredictResponse = client.predict(request).await?.into_inner();
    println!("Response from Half Plus Two Service:");
    println!(
        "OUTPUTS : {:#?}",
        response.outputs.get("y").unwrap().float_val
    );
    Ok(())
}
