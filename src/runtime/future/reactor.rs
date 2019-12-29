use alloc::{collections::VecDeque, rc::Rc};
use core::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Poll, RawWaker, RawWakerVTable, Waker},
};
use futures_core::{future::LocalBoxFuture, task::Context};
use generational_arena::{Arena, Index as TaskId};

// reactor lives in a thread local variable. Here's where all magic happens!
thread_local! {
    static REACTOR: EventLoop = EventLoop::new();
}

pub fn tick() -> bool {
    REACTOR.with(|reactor| reactor.tick())
}

pub fn spawn<F: Future<Output = ()> + 'static>(f: F) {
    REACTOR.with(|reactor| reactor.spawn(f))
}

// pub fn spawn_link<F: Future<Output = ()> + 'static>(f: F) -> Link {
//     REACTOR.with(|reactor| reactor.spawn_link(f))
// }

// Wakeup notification struct stores the index of the future in the wait queue
// and waker
#[derive(Debug)]
struct Wakeup {
    id: TaskId,
    waker: Waker,
}

// Task is a boxed future with Output = ()
struct Task {
    future: LocalBoxFuture<'static, ()>,
}

impl Task {
    // returning Ready will lead to task being removed from wait queues and dropped
    fn poll(&mut self, waker: Waker) -> Poll<()> {
        let future = Pin::new(&mut self.future);
        let mut ctx = Context::from_waker(&waker);

        match future.poll(&mut ctx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct EventLoop {
    wait_queue: RefCell<Arena<Task>>,
    run_queue: RefCell<VecDeque<Wakeup>>,
    new_queue: RefCell<VecDeque<Task>>,
}

impl EventLoop {
    fn new() -> Self {
        EventLoop {
            wait_queue: RefCell::new(Arena::new()),
            run_queue: RefCell::new(VecDeque::new()),
            new_queue: RefCell::new(VecDeque::new()),
        }
    }

    // waker calls this to put the future on the run queue
    fn wake(&self, wakeup: Wakeup) {
        self.run_queue.borrow_mut().push_back(wakeup);
    }

    // create a task, poll it once and push it on wait queue
    fn spawn<F: Future<Output = ()> + 'static>(&self, f: F) {
        self.new_queue.borrow_mut().push_back(Task {
            future: Box::pin(f),
        });
    }

    fn ingest_new_tasks(&self) -> bool {
        let mut has_entries = false;
        while let Some(task) = self.new_queue.borrow_mut().pop_front() {
            has_entries = true;
            self.run_queue
                .borrow_mut()
                .push_back(self.insert_task(task));
        }
        has_entries
    }

    fn insert_task(&self, task: Task) -> Wakeup {
        let id = self.wait_queue.borrow_mut().insert(task);
        let waker = token_waker(Rc::new(id));
        Wakeup { id, waker }
    }

    fn tick(&self) -> bool {
        loop {
            self.poll_run_queue();

            if !self.ingest_new_tasks() {
                break;
            }
        }

        self.has_work()
    }

    fn poll_run_queue(&self) {
        loop {
            let wakeup = if let Some(w) = self.run_queue.borrow_mut().pop_front() {
                w
            } else {
                break;
            };
            self.poll_wakeup(wakeup);
        }
    }

    fn poll_wakeup(&self, wakeup: Wakeup) {
        let id = wakeup.id;

        let should_remove = if let Some(task) = self.wait_queue.borrow_mut().get_mut(id) {
            Poll::Ready(()) == task.poll(wakeup.waker)
        } else {
            false
        };

        if should_remove {
            self.wait_queue.borrow_mut().remove(id);
        }
    }

    fn has_work(&self) -> bool {
        !self.wait_queue.borrow().is_empty() || !self.new_queue.borrow().is_empty()
    }
}

/// Creates a [`Waker`] from an `Arc<impl ArcWake>`.
///
/// The returned [`Waker`] will call
/// [`ArcWake.wake()`](ArcWake::wake) if awoken.
fn token_waker(token: Rc<TaskId>) -> Waker {
    let ptr = Rc::into_raw(token) as *const ();

    unsafe { Waker::from_raw(RawWaker::new(ptr, waker_vtable())) }
}

fn waker_vtable() -> &'static RawWakerVTable {
    &RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop)
}

#[allow(clippy::redundant_clone)] // The clone here isn't actually redundant.
unsafe fn increase_refcount(data: *const ()) {
    // Retain Arc, but don't touch refcount by wrapping in ManuallyDrop
    let rc = core::mem::ManuallyDrop::new(Rc::<TaskId>::from_raw(data as *const TaskId));
    // Now increase refcount, but don't drop new refcount either
    let _rc_clone = rc.clone();
}

// used by `waker_ref`
unsafe fn waker_clone(data: *const ()) -> RawWaker {
    increase_refcount(data);
    RawWaker::new(data, waker_vtable())
}

unsafe fn waker_wake(data: *const ()) {
    let token: Rc<TaskId> = Rc::from_raw(data as *const TaskId);

    REACTOR.with(|reactor| {
        reactor.wake(Wakeup {
            id: *token,
            waker: token_waker(token),
        })
    });
}

// used by `waker_ref`
unsafe fn waker_wake_by_ref(data: *const ()) {
    let token = core::mem::ManuallyDrop::new(Rc::<TaskId>::from_raw(data as *const TaskId));

    REACTOR.with(|reactor| {
        reactor.wake(Wakeup {
            id: **token,
            waker: Waker::from_raw(RawWaker::new(data, waker_vtable())),
        })
    });
}

unsafe fn waker_drop(data: *const ()) {
    drop(Rc::<TaskId>::from_raw(data as *const TaskId))
}
