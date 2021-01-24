use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

use once_cell::sync::Lazy;

pub(super) static FRAME_CHANGE_EVENT: Lazy<Mutex<FrameChangeEvent>> =
    Lazy::new(|| Mutex::new(FrameChangeEvent::new()));

pub(super) struct FrameChangeEvent {
    wakers: Vec<Waker>,
}
impl FrameChangeEvent {
    fn new() -> Self {
        Self { wakers: vec![] }
    }

    pub(super) fn update(&mut self) {
        for w in self.wakers.drain(..) {
            w.wake_by_ref();
        }
    }

    pub(super) fn register_callback(&mut self, waker: Waker) {
        self.wakers.push(waker);
    }
}

pub struct NextFrameFuture {
    called: bool,
}
impl NextFrameFuture {
    fn new() -> Self {
        Self { called: false }
    }
}
impl Future for NextFrameFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.called {
            Poll::Ready(())
        } else {
            self.called = true;
            let waker = cx.waker().clone();
            FRAME_CHANGE_EVENT.lock().unwrap().register_callback(waker);
            Poll::Pending
        }
    }
}

pub fn next_frame() -> impl Future {
    NextFrameFuture::new()
}
