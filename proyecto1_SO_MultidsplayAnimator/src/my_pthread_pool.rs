use std::ptr::null;
use libc::{clone, ucontext_t};
use crate::my_pthread::{MyPthread, schedulerEnum};

pub(crate) struct PthreadPool {
    pub(crate) scheduler: schedulerEnum,
    pub(crate) pthreads: Vec<*mut MyPthread>,
    pub(crate) rr_pthreads: Vec<*mut MyPthread>,
    pub(crate) lt_pthreads: Vec<*mut MyPthread>,
    pub(crate) rt_pthreads: Vec<*mut MyPthread>,
    pub(crate) actual_context: Option<ucontext_t>,
    pub(crate) serial: u32
}



pub(crate) fn create_pthread_pool() -> PthreadPool {
    let mut pool = PthreadPool {
        scheduler: schedulerEnum::round_robin,
        pthreads: Vec::new(),
        rr_pthreads: Vec::new(),
        lt_pthreads: Vec::new(),
        rt_pthreads: Vec::new(),
        actual_context: None,
        serial: 0
    };
    return pool
}

pub(crate) unsafe fn add_pthread(mut pool: PthreadPool, thread :*mut MyPthread) -> PthreadPool {
    pool.pthreads.push(thread);
    pool.serial += 1;
    match thread.sched {
        schedulerEnum::round_robin => {
            pool.rr_pthreads.push(thread)
        }
        schedulerEnum::lottery => {
            pool.lt_pthreads.push(thread);
        }
        schedulerEnum::real_time => {
            pool.rt_pthreads.push(thread);
        }
    }
    return pool;
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