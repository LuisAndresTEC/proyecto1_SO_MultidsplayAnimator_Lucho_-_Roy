use libc::{free, sleep, ucontext_t, usleep};
use crate::my_pthread::{my_thread_yield, MyPthread, SchedulerEnum};
use crate::my_pthread_pool::{PthreadPool, remove_thread};

pub(crate) fn  my_mutex_init()-> bool {
    let mutex = false;
    return mutex
}

pub(crate) fn change_mutex_state(mut mutex: Option<bool>) -> bool {
    match mutex {
        Some(true) => {
            mutex = Some(false);
            return true;
        }
        Some(false) => {
            mutex = Some(true);
            return false;
        }
        None => {
            return false;
        }
    }
}

pub(crate) unsafe fn my_mutex_lock(mut pool: PthreadPool) -> PthreadPool {
    pool.mutex = Option::from(change_mutex_state(pool.mutex));
    return (pool);
}

pub(crate) unsafe fn my_mutex_destroy(mut pool: PthreadPool) -> PthreadPool {
    pool.mutex = None;
    return pool;
}

//my_mutex_unlock
pub(crate) unsafe fn my_mutex_unlock(mut pool: PthreadPool) -> (PthreadPool) {
    pool.mutex = Option::from(change_mutex_state(pool.mutex));
    return (pool);
}

//my_mutex_trylock
pub(crate) unsafe fn my_mutex_trylock(mut pool: PthreadPool) -> (PthreadPool) {

    pool.mutex = Option::from(change_mutex_state(pool.mutex));
    return (pool);
}