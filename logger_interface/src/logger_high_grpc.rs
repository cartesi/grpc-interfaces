// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait LoggerManagerHigh {
    fn submit_file(&self, o: ::grpc::RequestOptions, p: super::logger_high::SubmitFileRequest) -> ::grpc::SingleResponse<super::cartesi_base::Hash>;

    fn download_file(&self, o: ::grpc::RequestOptions, p: super::logger_high::DownloadFileRequest) -> ::grpc::SingleResponse<super::logger_high::FilePath>;
}

// client

pub struct LoggerManagerHighClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_SubmitFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::logger_high::SubmitFileRequest, super::cartesi_base::Hash>>,
    method_DownloadFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::logger_high::DownloadFileRequest, super::logger_high::FilePath>>,
}

impl ::grpc::ClientStub for LoggerManagerHighClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LoggerManagerHighClient {
            grpc_client: grpc_client,
            method_SubmitFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/LoggerManagerHigh.LoggerManagerHigh/SubmitFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DownloadFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/LoggerManagerHigh.LoggerManagerHigh/DownloadFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl LoggerManagerHigh for LoggerManagerHighClient {
    fn submit_file(&self, o: ::grpc::RequestOptions, p: super::logger_high::SubmitFileRequest) -> ::grpc::SingleResponse<super::cartesi_base::Hash> {
        self.grpc_client.call_unary(o, p, self.method_SubmitFile.clone())
    }

    fn download_file(&self, o: ::grpc::RequestOptions, p: super::logger_high::DownloadFileRequest) -> ::grpc::SingleResponse<super::logger_high::FilePath> {
        self.grpc_client.call_unary(o, p, self.method_DownloadFile.clone())
    }
}

// server

pub struct LoggerManagerHighServer;


impl LoggerManagerHighServer {
    pub fn new_service_def<H : LoggerManagerHigh + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/LoggerManagerHigh.LoggerManagerHigh",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/LoggerManagerHigh.LoggerManagerHigh/SubmitFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.submit_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/LoggerManagerHigh.LoggerManagerHigh/DownloadFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.download_file(o, p))
                    },
                ),
            ],
        )
    }
}
