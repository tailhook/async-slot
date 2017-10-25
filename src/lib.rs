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

use std::fmt;
use std::error::Error;

/// Error type for sending, used when the receiving end of a channel is
/// dropped
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SendError<T>(T);

impl<T: fmt::Debug> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error sending value {:?} through a slot channel, \
                   receiver has gone", self.0)
    }
}

impl<T: fmt::Debug> Error for SendError<T> {
    fn description(&self) -> &str {
        "error sending value through a slot channel, receiver has gone"
    }
}
