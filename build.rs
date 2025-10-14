fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("src/generated")
        .file_descriptor_set_path("src/signals_descriptor.rs") // Output file for FILE_DESCRIPTOR_SET
        .compile_protos(&["proto/signals.proto"], &["proto"])?;
    Ok(())
}