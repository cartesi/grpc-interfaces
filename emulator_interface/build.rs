// Copyright 2019 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate protoc_rust_grpc;

use std::path::Path;

fn main() {
    let generated_files = [
        "src/cartesi_base.rs",
        "src/manager_high.rs",
        "src/manager_high_grpc.rs",
    ];
    if !generated_files.iter().all(|&f| Path::new(f).exists()) {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src",
            includes: &["../"],
            input: &["../cartesi-base.proto", "../manager-high.proto"],
            rust_protobuf: true, // generate protobuf messages, not just services
            ..Default::default()
        })
        .expect("protoc-rust-grpc");
    }
}
