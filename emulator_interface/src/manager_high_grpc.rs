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

pub trait MachineManagerHigh {
    fn new_session(&self, o: ::grpc::RequestOptions, p: super::manager_high::NewSessionRequest) -> ::grpc::SingleResponse<super::cartesi_base::Hash>;

    fn session_run(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionRunRequest) -> ::grpc::SingleResponse<super::manager_high::SessionRunResult>;

    fn session_step(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionStepRequest) -> ::grpc::SingleResponse<super::manager_high::SessionStepResult>;
}

// client

pub struct MachineManagerHighClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_NewSession: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::NewSessionRequest, super::cartesi_base::Hash>>,
    method_SessionRun: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionRunRequest, super::manager_high::SessionRunResult>>,
    method_SessionStep: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionStepRequest, super::manager_high::SessionStepResult>>,
}

impl ::grpc::ClientStub for MachineManagerHighClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MachineManagerHighClient {
            grpc_client: grpc_client,
            method_NewSession: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/NewSession".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SessionRun: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/SessionRun".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SessionStep: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/SessionStep".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl MachineManagerHigh for MachineManagerHighClient {
    fn new_session(&self, o: ::grpc::RequestOptions, p: super::manager_high::NewSessionRequest) -> ::grpc::SingleResponse<super::cartesi_base::Hash> {
        self.grpc_client.call_unary(o, p, self.method_NewSession.clone())
    }

    fn session_run(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionRunRequest) -> ::grpc::SingleResponse<super::manager_high::SessionRunResult> {
        self.grpc_client.call_unary(o, p, self.method_SessionRun.clone())
    }

    fn session_step(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionStepRequest) -> ::grpc::SingleResponse<super::manager_high::SessionStepResult> {
        self.grpc_client.call_unary(o, p, self.method_SessionStep.clone())
    }
}

// server

pub struct MachineManagerHighServer;


impl MachineManagerHighServer {
    pub fn new_service_def<H : MachineManagerHigh + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/CartesiManagerHigh.MachineManagerHigh",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/NewSession".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.new_session(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/SessionRun".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.session_run(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/SessionStep".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.session_step(o, p))
                    },
                ),
            ],
        )
    }
}
