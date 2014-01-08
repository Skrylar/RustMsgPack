
#[feature(globs)];

#[crate_type="rlib"];
#[crate_id="msgpack"];

mod magic;
mod testing;
pub mod reader;
pub mod writer;
