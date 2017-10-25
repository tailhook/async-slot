//! The crate provides an unbounded channel that only stores last value sent.
//!
//! There are two implementations a one is traditionally called `sync` in the
//! futures world. It works across threads.  And the `unsync` one, which only
//! works in single thread, but is potentially more performant.
//!
//! The `sync` one which should be used by default is also reexported at
//! the root of this crate.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

extern crate futures;

pub mod sync;
pub mod unsync;

pub use sync::{channel, Receiver, Sender};
