fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .file_descriptor_set_path("target/tmp")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
