use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use tensorflowlib::{
    model_spec, prediction_service_client::PredictionServiceClient, tensor_shape_proto::Dim,
    ModelSpec, PredictRequest, PredictResponse, TensorProto, TensorShapeProto,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_spec = ModelSpec {
        name: "half_plus_two".to_string(),
        signature_name: "serving_default".to_string(),
        version_choice: Some(model_spec::VersionChoice::Version(123)),
    };

    let total_inputs = 3i64;
    let tensor_dims = TensorShapeProto {
        dim: vec![
            Dim {
                size: total_inputs,
                name: "rows".to_string(),
            },
            Dim {
                size: 1i64,
                name: "dim1".to_string(),
            },
            Dim {
                size: 1i64,
                name: "dim2".to_string(),
            },
        ],
        unknown_rank: false,
    };
    let mut inputs = HashMap::<String, TensorProto>::new();
    let mut all_tensors: Vec<u8> = vec![];

    // fetch some random floats in the range 0..10.
    let step = Uniform::new(0.0, 10.0);
    let input_vec: Vec<f32> = step
        .sample_iter(&mut rand::thread_rng())
        .take(total_inputs as usize)
        .collect();

    println!("Calling Half Plus Two Service: (y = x*0.5 + 2)");
    println!("INPUTS: {:#?}", input_vec);
    for f in input_vec {
        all_tensors.extend_from_slice(&f.to_ne_bytes());
    }
    let tensor = TensorProto {
        dtype: 1i32,
        tensor_shape: Some(tensor_dims),
        version_number: 0i32,
        tensor_content: all_tensors,
        float_val: vec![0f32],
    };
    inputs.insert("x".to_string(), tensor);
    let request = tonic::Request::new(PredictRequest {
        model_spec: Some(model_spec),
        inputs: inputs,
        output_filter: vec!["y".to_string()],
    });
    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:8500")
        .connect()
        .await?;
    let mut client = PredictionServiceClient::new(channel);
    let response: PredictResponse = client.predict(request).await?.into_inner();
    println!(
        "OUTPUTS : {:#?}",
        response.outputs.get("y").unwrap().float_val
    );
    Ok(())
}
