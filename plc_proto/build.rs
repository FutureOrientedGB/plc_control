fn main() {
    println!("cargo:rerun-if-changed=src/");
    tonic_build::configure()
        .type_attribute("", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(true)
        .compile_protos(&["src/pb/rpc.proto"], &["src/pb"])
        .unwrap();
}