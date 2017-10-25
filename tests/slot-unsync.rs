extern crate futures;
extern crate async_slot;

use std::rc::Rc;
use futures::prelude::*;
use futures::future::{lazy};
use async_slot::unsync as slot;

#[test]
fn send_recv() {
    let (tx, rx) = slot::channel::<i32>();
    let mut rx = rx.wait();

    tx.send(1).wait().unwrap();

    assert_eq!(rx.next().unwrap(), Ok(1));
}

#[test]
fn swap() {
    let (tx, rx) = slot::channel::<i32>();
    let mut rx = rx.wait();

    assert_eq!(tx.swap(1), Ok(None));
    assert_eq!(tx.swap(2), Ok(Some(1)));
    assert_eq!(rx.next().unwrap(), Ok(2));
    assert_eq!(tx.swap(3), Ok(None));
    assert_eq!(rx.next().unwrap(), Ok(3));
}

#[test]
fn is_canceled() {
    let (tx, rx) = slot::channel::<i32>();
    let mut rx = rx.wait();

    assert_eq!(tx.swap(1), Ok(None));
    assert_eq!(rx.next().unwrap(), Ok(1));
    assert_eq!(tx.is_canceled(), false);
    drop(rx);
    assert_eq!(tx.is_canceled(), true);
    assert_eq!(tx.swap(2).unwrap_err().into_inner(), 2);
}

#[test]
fn poll_cancel() {
    let (mut tx, rx) = slot::channel::<i32>();
    let mut rx = rx.wait();

    lazy(move || {
        assert_eq!(tx.swap(1), Ok(None));
        assert_eq!(rx.next().unwrap(), Ok(1));
        assert_eq!(tx.poll_cancel(), Ok(Async::NotReady));
        drop(rx);
        assert_eq!(tx.poll_cancel(), Ok(Async::Ready(())));
        assert_eq!(tx.swap(2).unwrap_err().into_inner(), 2);
        Ok::<(), ()>(())
    }).wait().unwrap();
}

#[test]
fn send_recv_no_buffer() {
    let (mut tx, mut rx) = slot::channel::<i32>();

    // Run on a task context
    lazy(move || {
        assert!(tx.poll_complete().unwrap().is_ready());

        // Send first message

        let res = tx.start_send(1).unwrap();
        assert!(is_ready(&res));

        // Send second message
        let res = tx.start_send(2).unwrap();
        assert!(is_ready(&res));

        // Take the value
        assert_eq!(rx.poll().unwrap(), Async::Ready(Some(2)));

        let res = tx.start_send(3).unwrap();
        assert!(is_ready(&res));

        // Take the value
        assert_eq!(rx.poll().unwrap(), Async::Ready(Some(3)));

        Ok::<(), ()>(())
    }).wait().unwrap();
}

#[test]
fn send_shared_recv() {
    let (tx, rx) = slot::channel::<i32>();
    let tx1 = Rc::new(tx);
    let tx2 = tx1.clone();
    let mut rx = rx.wait();

    tx1.swap(1).unwrap();
    assert_eq!(rx.next().unwrap(), Ok(1));

    tx2.swap(2).unwrap();
    assert_eq!(rx.next().unwrap(), Ok(2));
}

#[test]
fn tx_close_gets_none() {
    let (_, mut rx) = slot::channel::<i32>();

    // Run on a task context
    lazy(move || {
        assert_eq!(rx.poll(), Ok(Async::Ready(None)));
        assert_eq!(rx.poll(), Ok(Async::Ready(None)));

        Ok::<(), ()>(())
    }).wait().unwrap();
}

fn is_ready<T>(res: &AsyncSink<T>) -> bool {
    match *res {
        AsyncSink::Ready => true,
        _ => false,
    }
}

