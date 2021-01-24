use std::collections::{BTreeMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::task::ArcWake;
use once_cell::sync::Lazy;

use super::handle::JoinHandle;
use super::next_frame_future::FRAME_CHANGE_EVENT;
use super::task::{joinable, Task};

static RUNNING_QUEUE: Lazy<Mutex<VecDeque<Arc<Mutex<Task>>>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));
static WAIT_QUEUE: Lazy<Mutex<BTreeMap<usize, Arc<Mutex<Task>>>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));
static TASK_COUNTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let (task, handle) = joinable(future);
    RUNNING_QUEUE.lock().unwrap().push_back(task);
    handle
}

pub fn runtime_update() {
    println!("---- Frame start ----");
    {
        println!(
            "Remain tasks num: {}",
            RUNNING_QUEUE.lock().unwrap().iter().count()
        );
    }
    {
        println!(
            "Remain wait tasks num: {}",
            WAIT_QUEUE.lock().unwrap().iter().count()
        );
    }

    {
        FRAME_CHANGE_EVENT.lock().unwrap().update();
    }

    'current_frame: loop {
        let task = { RUNNING_QUEUE.lock().unwrap().pop_front() };

        match task {
            None => break 'current_frame,
            Some(task) => {
                let id = {
                    let mut counter = TASK_COUNTER.lock().unwrap();
                    let id = *counter;
                    *counter = id + 1;
                    id
                };

                let flag = Arc::new(AtomicBool::new(false));

                let waker = TaskWaker::waker(Arc::clone(&task), Arc::clone(&flag), id);
                let mut cx = Context::from_waker(&waker);

                let pending = {
                    let mut task = task.lock().unwrap();
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
                    WAIT_QUEUE.lock().unwrap().insert(id, task);
                }
            }
        }
    }
}

struct TaskWaker {
    task: Mutex<Option<Arc<Mutex<Task>>>>,
    flag: Arc<AtomicBool>,
    task_id: usize,
}
impl TaskWaker {
    fn waker(task: Arc<Mutex<Task>>, flag: Arc<AtomicBool>, task_id: usize) -> Waker {
        futures::task::waker(Arc::new(TaskWaker {
            task: Mutex::new(Some(task)),
            flag,
            task_id,
        }))
    }
}
impl ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // flagをたてる
        arc_self.flag.store(true, Ordering::Relaxed);

        if let Some(task) = arc_self.task.lock().unwrap().take() {
            // taskをRunning Queueについか
            let mut q = RUNNING_QUEUE.lock().unwrap();
            q.push_back(task);

            // taskをWait Queueから取り除く
            WAIT_QUEUE.lock().unwrap().remove(&arc_self.task_id);
        }
    }
}
