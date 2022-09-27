
use crate::my_pthread::MyPthread;

pub(crate) struct PthreadPool {
    pub(crate) pthreads: Vec<MyPthread>,
    pub(crate) serial: u32
}



pub(crate) fn create_pthread_pool() -> PthreadPool {
    let mut pool = PthreadPool {
        pthreads: Vec::new(),
        serial: 0
    };
    return pool
}

pub(crate) fn add_pthread(pool: PthreadPool, thread :MyPthread) -> PthreadPool {
    let mut pool = pool;
    pool.pthreads.push(thread);
    pool.serial += 1;
    return pool
}