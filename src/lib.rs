// NOTE: The reexport of `macro_magic` crate does not work (I do not know why), so users still
// have to specify the dependency in their Cargo.toml :(
pub extern crate macro_magic;
pub use macro_magic::*;
pub use generics_alias_macros::*;
