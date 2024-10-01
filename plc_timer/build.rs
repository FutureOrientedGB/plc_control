use plc_build;

fn main() {
    println!("cargo:rerun-if-changed=./");
    println!("cargo:rerun-if-changed=../plc_build/");
    
    plc_build::replace_app_name_version()
}

