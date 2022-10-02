use std::ptr::null;
use libc::ucontext_t;
use crate::my_pthread::MyPthread;

pub(crate) struct PthreadPool {
    pub(crate) pthreads: Vec<MyPthread>,
    pub(crate) contexts: Vec<ucontext_t>,
    pub(crate) actualContext: Option<ucontext_t>,
    pub(crate) serial: u32
}



pub(crate) fn create_pthread_pool() -> PthreadPool {
    let mut pool = PthreadPool {
        pthreads: Vec::new(),
        contexts: Vec::new(),
        actualContext: None,
        serial: 0
    };
    return pool
}

pub(crate) fn add_pthread(pool: PthreadPool, thread :MyPthread) -> PthreadPool {
    let mut pool = pool;
    pool.pthreads.push(thread);
    pool.contexts.push(pool.pthreads[pool.pthreads.len()-1].context);
    pool.serial += 1;
    return pool
}

pub(crate) fn remove_thread(pool: PthreadPool, index: usize) -> PthreadPool {
    let mut pool = pool;
    pool.pthreads.remove(index);
    pool.contexts.remove(index);
    return pool
}

pub(crate) fn set_transit_values(mut pool: PthreadPool, aux: usize) -> PthreadPool {
    let mut thread = &pool.pthreads[aux];
    pool.actualContext = Some(thread.context);
    return pool
}