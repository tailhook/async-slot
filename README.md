Async Slot
==========

[Documentation](https://docs.rs/async-slot) |
[Github](https://github.com/tailhook/async-slot) |
[Crate](https://crates.io/crates/async-slot)

An unbounded channel that only stores last value sent made for [futures].

[futures]: https://github.com/alexcrichton/futures-rs

Features:

1. Compacts memory (only last value is kept)
2. Has `poll_cancel` and `is_canceled`
3. Single-producer/single-consumer
4. Never has backpressure (because value is just replaced)
5. Replaced value can be recovered if using `swap` method.
6. `Sync`, so if multi-producer is desired, `Sender` can be wrapped
   into an `Arc`/`Rc` and `swap` method is used to update value.

License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

