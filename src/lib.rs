mod obj;

// TODO derive as much of the following as possible on all structs and enums:
// Debug, Eq, PartialEq, Default, Clone, Copy
// TODO response packet
// TODO reponse packet builder
pub mod packet;

// TODO parser for String -> Reponse/Request packet

pub use obj::{
    Version,
    Header,
    StatusCode,
    StatusCodeInt,
    Body,
    Method,
};
