//! A pluggable, mockable process invocation abstraction for Rust.

mod child;
pub use child::Child;

mod command;
pub use command::Command;

mod context;
pub use context::Context;

mod output;
pub use output::Output;

pub mod runner;

mod stdio;
pub use stdio::*;
