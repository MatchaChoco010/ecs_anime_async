use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, VecDeque};
use std::future::Future;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::thread_local;

use futures::task::ArcWake;

use super::handle::JoinHandle;
use super::next_frame_future::FRAME_CHANGE_EVENT;
use super::task::{joinable, Task};

thread_local! {
    static RUNNING_QUEUE: RefCell<VecDeque<Rc<RefCell<Task>>>> =
        RefCell::new(VecDeque::new());
    static WAIT_QUEUE: RefCell<BTreeMap<usize, Rc<RefCell<Task>>>> =
        RefCell::new(BTreeMap::new());
    static TASK_COUNTER: Cell<usize> = Cell::new(0);
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    let (task, handle) = joinable(future);
    RUNNING_QUEUE.with(|q| q.borrow_mut().push_back(task));
    handle
}

pub fn runtime_update() {
    FRAME_CHANGE_EVENT.with(|ev| ev.borrow_mut().update());

    'current_frame: loop {
        let task = { RUNNING_QUEUE.with(|q| q.borrow_mut().pop_front()) };

        match task {
            None => break 'current_frame,
            Some(task) => {
                let id = {
                    TASK_COUNTER.with(|counter| {
                        let id = counter.get();
                        counter.set(id + 1);
                        id
                    })
                };

                let flag = Arc::new(AtomicBool::new(false));

                let waker = TaskWaker::waker(Arc::clone(&flag), id);
                let mut cx = Context::from_waker(&waker);

                let pending = {
                    let mut task = task.borrow_mut();
                    match task.poll(&mut cx) {
                        // taskの完了をJoinHandleに通知する
                        Poll::Ready(()) => {
                            task.callback_waker.iter().for_each(|w| w.wake_by_ref());
                            false
                        }
                        Poll::Pending => true,
                    }
                };
                // Pendingでなおかつ即座に実行可能でない場合はParkする
                if pending && !flag.load(Ordering::Relaxed) {
                    WAIT_QUEUE.with(|q| q.borrow_mut().insert(id, task));
                }
            }
        }
    }
}

struct TaskWaker {
    flag: Arc<AtomicBool>,
    task_id: usize,
}
impl TaskWaker {
    fn waker(flag: Arc<AtomicBool>, task_id: usize) -> Waker {
        futures::task::waker(Arc::new(TaskWaker { flag, task_id }))
    }
}
impl ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // flagをたてる
        arc_self.flag.store(true, Ordering::Relaxed);

        let task = WAIT_QUEUE.with(|q| q.borrow_mut().remove(&arc_self.task_id));
        if let Some(task) = task {
            RUNNING_QUEUE.with(|q| {
                let mut q = q.borrow_mut();
                q.push_back(task);
            });
        }
    }
}
