use std::ptr::null;
use libc::{clone, ucontext_t};
use crate::mutex::{MyMutex,my_mutex_init};
use crate::my_pthread::{MyPthread, schedulerEnum};

#[derive(Clone)]
pub(crate) struct PthreadPool {
    pub(crate) scheduler: schedulerEnum,
    pub(crate) pthreads: Vec<MyPthread>,
    pub(crate) rr_pthreads: Vec<MyPthread>,
    pub(crate) lt_pthreads: Vec<MyPthread>,
    pub(crate) rt_pthreads: Vec<MyPthread>,
    pub(crate) actual_thread: Vec<MyPthread>,
    pub(crate) mutex: MyMutex,
    pub(crate) serial: u32
}



pub(crate) fn create_pthread_pool() -> PthreadPool {
    let mut pool = PthreadPool {
        scheduler: schedulerEnum::round_robin,
        pthreads: Vec::new(),
        rr_pthreads: Vec::new(),
        lt_pthreads: Vec::new(),
        rt_pthreads: Vec::new(),
        actual_thread: Vec::new(),
        mutex: my_mutex_init(),
        serial: 0
    };
    return pool
}

pub(crate) fn remove_thread(pool: PthreadPool, index: usize) -> PthreadPool {
    let mut pool = pool;
    pool.pthreads.remove(index);
    return pool
}


pub(crate) fn change_scheduler(mut pool: PthreadPool, scheduler: schedulerEnum) -> PthreadPool {
    pool.scheduler = scheduler;
    return pool
}