pub mod rs;
pub use rs::*;

pub mod plc {
    tonic::include_proto!("plc");
}
