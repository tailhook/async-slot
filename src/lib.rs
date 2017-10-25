//! The crate provides an unbounded channel that only stores last value sent.
//!
//! There are two implementations a one is traditionally called `sync` in the
//! futures world. It works across threads.  And the `unsync` one, which only
//! works in single thread, but is potentially more performant.
//!
//! The `sync` one which should be used by default is also reexported at
//! the root of this crate.
//!
//! # Features
//!
//! 1. Compacts memory (only last value is kept)
//! 2. Has `poll_cancel` and `is_canceled`
//! 3. Single-producer/single-consumer
//! 4. Never has backpressure (because value is just replaced)
//! 5. Replaced value can be recovered if using `swap` method.
//! 6. `Sync`, so if multi-producer is desired, `Sender` can be wrapped
//!    into an `Arc`/`Rc` and `swap` method is used to update value.
//!
//! # Example
//!
//! ```
//! # extern crate futures;
//! # extern crate async_slot;
//! #
//! # use futures::prelude::*;
//! # use futures::stream::iter_ok;
//! #
//! # fn main() {
//! let (tx, rx) = async_slot::channel::<i32>();
//!
//! tx.send_all(iter_ok(vec![1, 2, 3])).wait();
//!
//! let received = rx.collect().wait().unwrap();
//! assert_eq!(received, vec![3]);
//! # }
//! ```
//!

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
