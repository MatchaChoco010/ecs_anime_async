use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::channel::oneshot;

use super::JoinHandle;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    pub(super) callback_waker: Option<Waker>,
}
impl Task {
    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        Future::poll(self.future.as_mut(), cx)
    }

    pub(super) fn register_callback(&mut self, waker: Waker) {
        self.callback_waker = Some(waker);
    }
}

pub(super) fn joinable<F>(future: F) -> (Arc<Mutex<Task>>, JoinHandle<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let (sender, receiver) = oneshot::channel();

    let task = Arc::new(Mutex::new(Task {
        future: Box::pin(async {
            let _ = sender.send(future.await);
        }),
        callback_waker: None,
    }));

    let handle = JoinHandle {
        receiver,
        task: Arc::clone(&task),
    };

    (task, handle)
}
