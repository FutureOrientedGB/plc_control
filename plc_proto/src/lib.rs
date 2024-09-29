pub mod rs;
pub use rs::*;

pub mod plc {
    tonic::include_proto!("plc");
}


pub fn get_version() -> String {
    let version = "851bc42.20240929.164929";
    return version.to_string();
}
