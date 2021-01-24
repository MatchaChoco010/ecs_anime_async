use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use super::JoinHandle;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
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
    F: Future + 'static,
    F::Output: 'static,
{
    let value = Rc::new(RefCell::new(None));

    let task = {
        let value = Rc::clone(&value);
        Arc::new(Mutex::new(Task {
            future: Box::pin(async move {
                let mut value = value.borrow_mut();
                *value = Some(future.await);
            }),
            callback_waker: None,
        }))
    };

    let handle = JoinHandle {
        value,
        task: Arc::clone(&task),
    };

    (task, handle)
}
