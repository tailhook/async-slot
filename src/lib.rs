#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

extern crate futures;

pub mod sync;
pub mod unsync;

pub use sync::{channel, Receiver, Sender};
