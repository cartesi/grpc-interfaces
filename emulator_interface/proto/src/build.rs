extern crate protoc_rust_grpc;

fn main() {
    //match
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["../../"],
        input: &[
            "../../cartesi-base.proto",
            "../../manager-high.proto",
        ],
        rust_protobuf: true, // generate protobuf messages, not just services
        ..Default::default()
    })
    .expect("protoc-rust-grpc");
}
