fn main() {
    println!("cargo:rerun-if-changed=src/plc.proto");
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(true)
        .compile(&["src/plc.proto"], &["proto"])
        .unwrap();
}