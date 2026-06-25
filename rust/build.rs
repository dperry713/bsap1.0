fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/grpc")
        .compile(&["../../proto/bsap.proto"], &["../../proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protobuf: {}", e));
    
    println!("cargo:rerun-if-changed=../../proto/bsap.proto");
}
