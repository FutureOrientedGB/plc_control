use plc_build;

fn main() {
    println!("cargo:rerun-if-changed=./");
    println!("cargo:rerun-if-changed=../plc_build/");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(true)
        .compile_protos(&["src/pb/rpc.proto"], &["src/pb"])
        .unwrap();

    plc_build::replace_app_name_version();
}
