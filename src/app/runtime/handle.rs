use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use futures::channel::oneshot::Receiver;

use super::task::Task;

pub struct JoinHandle<T> {
    pub(super) receiver: Receiver<T>,
    pub(super) task: Arc<Mutex<Task>>,
}
impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(val) = self.receiver.try_recv().unwrap() {
            Poll::Ready(val)
        } else {
            let waker = cx.waker().clone();
            self.task.lock().unwrap().register_callback(waker);
            Poll::Pending
        }
    }
}
