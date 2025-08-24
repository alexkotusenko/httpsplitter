mod obj;

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
