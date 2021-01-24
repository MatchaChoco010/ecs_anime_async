use std::collections::{BTreeMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread_local;

use futures::task::ArcWake;

use super::handle::JoinHandle;
use super::next_frame_future::FRAME_CHANGE_EVENT;
use super::task::{joinable, Task};

thread_local! {
    static RUNNING_QUEUE: Mutex<VecDeque<Arc<Mutex<Task>>>> =
        Mutex::new(VecDeque::new());
    static WAIT_QUEUE: Mutex<BTreeMap<usize, Arc<Mutex<Task>>>> =
        Mutex::new(BTreeMap::new());
    static TASK_COUNTER: Mutex<usize> = Mutex::new(0);
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    let (task, handle) = joinable(future);
    RUNNING_QUEUE.with(|q| q.lock().unwrap().push_back(task));
    handle
}

pub fn runtime_update() {
    // println!("---- Frame start ----");
    // {
    //     println!(
    //         "Remain tasks num: {}",
    //         RUNNING_QUEUE.with(|q| q.lock().unwrap().iter().count())
    //     );
    // }
    // {
    //     println!(
    //         "Remain wait tasks num: {}",
    //         WAIT_QUEUE.with(|q| q.lock().unwrap().iter().count())
    //     );
    // }

    {
        FRAME_CHANGE_EVENT.lock().unwrap().update();
    }

    'current_frame: loop {
        let task = { RUNNING_QUEUE.with(|q| q.lock().unwrap().pop_front()) };

        match task {
            None => break 'current_frame,
            Some(task) => {
                let id = {
                    TASK_COUNTER.with(|counter| {
                        let mut counter = counter.lock().unwrap();
                        let id = *counter;
                        *counter = id + 1;
                        id
                    })
                };

                let flag = Arc::new(AtomicBool::new(false));

                let waker = TaskWaker::waker(Arc::clone(&flag), id);
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
                    WAIT_QUEUE.with(|q| q.lock().unwrap().insert(id, task));
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

        // if let Some(task) = arc_self.task.lock().unwrap().take() {
        //     // taskをRunning Queueについか
        //     let mut q = RUNNING_QUEUE.lock().unwrap();
        //     q.push_back(task);

        //     // taskをWait Queueから取り除く
        // }
        let task = WAIT_QUEUE.with(|q| q.lock().unwrap().remove(&arc_self.task_id));
        if let Some(task) = task {
            RUNNING_QUEUE.with(|q| {
                let mut q = q.lock().unwrap();
                q.push_back(task);
            });
        }
    }
}
