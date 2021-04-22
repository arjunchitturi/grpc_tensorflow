/// Specifies one or more fully independent input Examples.
/// See examples at:
///     https://github.com/tensorflow/tensorflow/blob/master/example.proto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleList {
    #[prost(message, repeated, tag = "1")]
    pub examples: ::prost::alloc::vec::Vec<super::Example>,
}
/// Specifies one or more independent input Examples, with a common context
/// Example.
///
/// The common use case for context is to cleanly and optimally specify some
/// features that are common across multiple examples.
///
/// See example below with a search query as the context and multiple restaurants
/// to perform some inference on.
///
/// context: {
///   features: {
///     feature: {
///       key  : "query"
///       value: {
///         bytes_list: {
///           value: [ "pizza" ]
///         }
///       }
///     }
///   }
/// }
/// examples: {
///   features: {
///     feature: {
///       key  : "cuisine"
///       value: {
///         bytes_list: {
///           value: [ "Pizzeria" ]
///         }
///       }
///     }
///   }
/// }
/// examples: {
///   features: {
///     feature: {
///       key  : "cuisine"
///       value: {
///         bytes_list: {
///           value: [ "Taqueria" ]
///         }
///       }
///     }
///   }
/// }
///
/// Implementations of ExampleListWithContext merge the context Example into each
/// of the Examples. Note that feature keys must not be duplicated between the
/// Examples and context Example, or the behavior is undefined.
///
/// See also:
///     example.proto
///     https://developers.google.com/protocol-buffers/docs/proto3#maps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleListWithContext {
    #[prost(message, repeated, tag = "1")]
    pub examples: ::prost::alloc::vec::Vec<super::Example>,
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<super::Example>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(oneof = "input::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<input::Kind>,
}
/// Nested message and enum types in `Input`.
pub mod input {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        ExampleList(super::ExampleList),
        #[prost(message, tag = "2")]
        ExampleListWithContext(super::ExampleListWithContext),
    }
}
/// Metadata for an inference request such as the model name and version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelSpec {
    /// Required servable name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A named signature to evaluate. If unspecified, the default signature will
    /// be used.
    #[prost(string, tag = "3")]
    pub signature_name: ::prost::alloc::string::String,
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[prost(oneof = "model_spec::VersionChoice", tags = "2, 4")]
    pub version_choice: ::core::option::Option<model_spec::VersionChoice>,
}
/// Nested message and enum types in `ModelSpec`.
pub mod model_spec {
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VersionChoice {
        /// Use this specific version number.
        #[prost(message, tag = "2")]
        Version(i64),
        /// Use the version associated with the given label.
        #[prost(string, tag = "4")]
        VersionLabel(::prost::alloc::string::String),
    }
}
/// A single class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// Label or name of the class.
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// Score for this class (e.g., the probability the item belongs to this
    /// class). As per the proto3 default-value semantics, if the score is missing,
    /// it should be treated as 0.
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// List of classes for a single item (tensorflow.Example).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Classifications {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
}
/// Contains one result per input example, in the same order as the input in
/// ClassificationRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResult {
    #[prost(message, repeated, tag = "1")]
    pub classifications: ::prost::alloc::vec::Vec<Classifications>,
}
// RPC Interfaces

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResponse {
    /// Effective Model Specification used for classification.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Result of the classification.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ClassificationResult>,
}
/// Message returned for "signature_def" field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureDefMap {
    #[prost(map = "string, message", tag = "1")]
    pub signature_def:
        ::std::collections::HashMap<::prost::alloc::string::String, super::SignatureDef>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataRequest {
    /// Model Specification indicating which model we are querying for metadata.
    /// If version is not specified, will use the latest (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Metadata fields to get. Currently supported: "signature_def".
    #[prost(string, repeated, tag = "2")]
    pub metadata_field: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataResponse {
    /// Model Specification indicating which model this metadata belongs to.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Map of metadata field name to metadata field. The options for metadata
    /// field name are listed in GetModelMetadataRequest. Currently supported:
    /// "signature_def".
    #[prost(map = "string, message", tag = "2")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
}
/// Regression result for a single item (tensorflow.Example).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Regression {
    #[prost(float, tag = "1")]
    pub value: f32,
}
/// Contains one result per input example, in the same order as the input in
/// RegressionRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResult {
    #[prost(message, repeated, tag = "1")]
    pub regressions: ::prost::alloc::vec::Vec<Regression>,
}
// RPC interfaces.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResponse {
    /// Effective Model Specification used for regression.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<RegressionResult>,
}
/// Inference request such as classification, regression, etc...
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceTask {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    /// All ModelSpecs in a MultiInferenceRequest must access the same model name.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Signature's method_name. Should be one of the method names defined in
    /// third_party/tensorflow/python/saved_model/signature_constants.py.
    /// e.g. "tensorflow/serving/classify".
    #[prost(string, tag = "2")]
    pub method_name: ::prost::alloc::string::String,
}
/// Inference result, matches the type of request or is an error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceResult {
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(oneof = "inference_result::Result", tags = "2, 3")]
    pub result: ::core::option::Option<inference_result::Result>,
}
/// Nested message and enum types in `InferenceResult`.
pub mod inference_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        ClassificationResult(super::ClassificationResult),
        #[prost(message, tag = "3")]
        RegressionResult(super::RegressionResult),
    }
}
/// Inference request containing one or more requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceRequest {
    /// Inference tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<InferenceTask>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
/// Inference request containing one or more responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceResponse {
    /// List of results; one for each InferenceTask in the request, returned in the
    /// same order as the request.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<InferenceResult>,
}
/// PredictRequest specifies which TensorFlow model to run, as well as
/// how inputs are mapped to tensors and how outputs are filtered before
/// returning to user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input tensors.
    /// Names of input tensor are alias names. The mapping from aliases to real
    /// input tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'inputs' field.
    #[prost(map = "string, message", tag = "2")]
    pub inputs: ::std::collections::HashMap<::prost::alloc::string::String, super::TensorProto>,
    /// Output filter.
    /// Names specified are alias names. The mapping from aliases to real output
    /// tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'outputs' field.
    /// Only tensors specified here will be run/fetched and returned, with the
    /// exception that when none is specified, all tensors specified in the
    /// named signature will be run/fetched and returned.
    #[prost(string, repeated, tag = "3")]
    pub output_filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response for PredictRequest on successful run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// Effective Model Specification used to process PredictRequest.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Output tensors.
    #[prost(map = "string, message", tag = "1")]
    pub outputs: ::std::collections::HashMap<::prost::alloc::string::String, super::TensorProto>,
}
#[doc = r" Generated client implementations."]
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " open source marker; do not remove"]
    #[doc = " PredictionService provides access to machine-learned models loaded by"]
    #[doc = " model_servers."]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PredictionServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PredictionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Classify."]
        pub async fn classify(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassificationRequest>,
        ) -> Result<tonic::Response<super::ClassificationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Classify",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Regress."]
        pub async fn regress(
            &mut self,
            request: impl tonic::IntoRequest<super::RegressionRequest>,
        ) -> Result<tonic::Response<super::RegressionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Regress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Predict -- provides access to loaded TensorFlow model."]
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MultiInference API for multi-headed models."]
        pub async fn multi_inference(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiInferenceRequest>,
        ) -> Result<tonic::Response<super::MultiInferenceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/MultiInference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetModelMetadata - provides access to metadata for loaded models."]
        pub async fn get_model_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelMetadataRequest>,
        ) -> Result<tonic::Response<super::GetModelMetadataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/GetModelMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PredictionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PredictionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PredictionServiceClient {{ ... }}")
        }
    }
}
