use libc::ucontext_t;
use crate::my_pthread::{my_thread_yield, MyPthread, schedulerEnum};
use crate::my_pthread_pool::{PthreadPool, remove_thread};

#[derive(Clone)]
pub(crate) struct MyMutex {
    pub(crate) locked: bool,
    pub(crate) owner: Option<ucontext_t>,
    pub(crate) waiters: Vec<ucontext_t>
}

pub(crate) fn  my_mutex_init()-> MyMutex {
    let mut mutex = MyMutex {
        locked: false,
        owner: None,
        waiters: Vec::new()
    };
    return mutex
}

pub(crate) unsafe fn my_mutex_lock(mut mutex: MyMutex, mut pool: PthreadPool) -> (MyMutex, PthreadPool) {
    if mutex.locked {

        mutex.waiters.push(pool.actual_thread[0].context);
        pool = my_thread_yield(pool);
        return (mutex, pool)
    } else {
        mutex.locked = true;
        mutex.owner = Some(pool.actual_thread[0].context);
        return (mutex, pool)
    }
}

pub(crate) fn my_mutex_destroy(mut pool: PthreadPool) -> (PthreadPool) {
    let mut waiters = pool.mutex.waiters;
    let mut mutex = MyMutex {
        locked: false,
        owner: None,
        waiters: waiters
    };
    pool.mutex = mutex;
    return pool
}

//my_mutex_unlock
pub(crate) fn my_mutex_unlock( mut pool: PthreadPool) -> (PthreadPool) {
    if pool.mutex.waiters.len() > 0 {
        let mut waiters = pool.mutex.waiters;
        let mut new_owner = waiters.remove(0);
        pool.mutex.owner = Some(new_owner);
        pool.mutex.waiters = waiters;
        return pool;
    } else {
        pool.mutex.locked = false;
        pool.mutex.owner = None;
        return pool;
    }
}

//my_mutex_trylock
pub(crate) fn my_mutex_trylock(mut pool: PthreadPool) -> (PthreadPool) {
    if pool.mutex.locked {
        return pool;
    } else {
        pool.mutex.locked = true;
        pool.mutex.owner = Some(pool.actual_thread[0].context);
        return pool;
    }
}