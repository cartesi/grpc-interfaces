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

    fn session_read_memory(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionReadMemoryRequest) -> ::grpc::SingleResponse<super::manager_high::SessionReadMemoryResult>;

    fn session_write_memory(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionWriteMemoryRequest) -> ::grpc::SingleResponse<super::cartesi_base::Void>;

    fn session_get_proof(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionGetProofRequest) -> ::grpc::SingleResponse<super::cartesi_base::Proof>;
}

// client

pub struct MachineManagerHighClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_NewSession: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::NewSessionRequest, super::cartesi_base::Hash>>,
    method_SessionRun: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionRunRequest, super::manager_high::SessionRunResult>>,
    method_SessionStep: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionStepRequest, super::manager_high::SessionStepResult>>,
    method_SessionReadMemory: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionReadMemoryRequest, super::manager_high::SessionReadMemoryResult>>,
    method_SessionWriteMemory: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionWriteMemoryRequest, super::cartesi_base::Void>>,
    method_SessionGetProof: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::manager_high::SessionGetProofRequest, super::cartesi_base::Proof>>,
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
            method_SessionReadMemory: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/SessionReadMemory".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SessionWriteMemory: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/SessionWriteMemory".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SessionGetProof: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CartesiManagerHigh.MachineManagerHigh/SessionGetProof".to_string(),
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

    fn session_read_memory(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionReadMemoryRequest) -> ::grpc::SingleResponse<super::manager_high::SessionReadMemoryResult> {
        self.grpc_client.call_unary(o, p, self.method_SessionReadMemory.clone())
    }

    fn session_write_memory(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionWriteMemoryRequest) -> ::grpc::SingleResponse<super::cartesi_base::Void> {
        self.grpc_client.call_unary(o, p, self.method_SessionWriteMemory.clone())
    }

    fn session_get_proof(&self, o: ::grpc::RequestOptions, p: super::manager_high::SessionGetProofRequest) -> ::grpc::SingleResponse<super::cartesi_base::Proof> {
        self.grpc_client.call_unary(o, p, self.method_SessionGetProof.clone())
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
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/SessionReadMemory".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.session_read_memory(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/SessionWriteMemory".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.session_write_memory(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CartesiManagerHigh.MachineManagerHigh/SessionGetProof".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.session_get_proof(o, p))
                    },
                ),
            ],
        )
    }
}
