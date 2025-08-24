/// Supported HTTP versions
pub enum Version {
    _0_9,
    _1_0,
    _1_1
}

impl Version {
    pub fn try_into(s: &str) -> Option<Self> {
        todo!();
    }
}


/// A header 
struct Header{} // TODO
