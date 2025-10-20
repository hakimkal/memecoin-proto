fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("src/generated")
        .type_attribute("signals.BuySignal", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("signals.TokenSecurityInfo", "#[derive(serde::Serialize, serde::Deserialize)]")
        .file_descriptor_set_path("src/signals_descriptor.bin") // Output file for FILE_DESCRIPTOR_SET
        .compile_protos(&["proto/signals.proto"], &["proto"])?;
    Ok(())
}