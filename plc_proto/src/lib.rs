pub mod rs;
pub use rs::*;

pub mod version;

pub mod plc {
    tonic::include_proto!("plc");
}


pub fn get_version() -> String {
    return String::from(version::GIT_COMMIT_VERSION);
}
