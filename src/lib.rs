//! Owner Criome Interface.
//!
//! ethos/interface.ethos is a strict role-free bootstrap Interface. Its
//! imports resolve through the ordinary producer's published Ethos directory,
//! and its Rust projection uses only authority-verified encoded coordinates.

pub mod bootstrap_manifest;
pub mod schema;

pub const META_CRIOME_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const META_CRIOME_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
