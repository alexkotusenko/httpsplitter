## httsplitter

This is a low-level crate for working with HTTP packets.

Use cases:
- Building request and reponse packets header by header
- Reading packets from streams
- Validating packets

### Example usage

Creating a HTTP reponse packet
```rust
use httpsplitter::packet::{RequestPacket, RequestPacketBuilder};
let req_p: RequestPacket = RequestPacketBuilder::new()
    .version(httpsplitter::Version::V1_1)
    .url("/")
    .method(httpsplitter::Method::Get)
    .try_build()
    .unwrap();
println!("{}", req_p.to_string());
```

### Versioning

This crate is **unstable**. Expect breaking changes at any point until the v1 release. That being said, most breaking changes will occur between minor versions (e.g. when updating from 0.5.0 to 0.6.0), and the patches will be kept as stable as possible.

### Scope & Goals

This crate is meant to serve as a low-level multitool for working with HTTP packets. Most crates abstract packet building away (and most likely you'd be better off using them instead), but sometimes it's helpful to have full control over your packets, and that's the pain point that this crate is trying to solve.

### Depencency Notes

This crate will be kept as dependency-free as possible.

### Contributing

All kinds of contributions are welcome, but please open an issue if you'd like to implement new features.
