mod obj;

// TODO derive as much of the following as possible on all structs and enums:
// Debug, Eq, PartialEq, Default, Clone, Copy
/// Packet structures and builders
pub mod packet;

pub use packet::PacketErr;

// TODO parser for String -> Reponse/Request packet

// TODO check if \r\n is in the right order  (adn not the inverse)

pub use obj::{
    Version,
    Header,
    StatusCode,
    StatusCodeInt,
    Body,
    Method,
};
