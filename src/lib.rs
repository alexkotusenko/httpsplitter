mod obj;

/// Packet structures and builders
pub mod packet;

/// Reading and collecting packets from streams. `reader` feature needed.
#[cfg(feature = "reader")]
pub mod reader;

pub use packet::PacketErr;

pub use obj::{
    Version,
    Header,
    StatusCode,
    StatusCodeInt,
    Body,
    Method,
};
