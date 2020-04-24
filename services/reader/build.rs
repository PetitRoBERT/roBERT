use tonic_build;

// This building script generate the necessary files
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "protos/reader.proto";
    let output_folder = "src/protos/";
    println!(
        "cargo:rerun-if-changed={} Generating gRPC files...",
        proto_root
    );
    tonic_build::configure()
        .out_dir(&output_folder)
        .compile(&["protos/reader.proto"], &["protos/"])
        .expect("Failed to compile gRPC definitions!");
    Ok(())
}
