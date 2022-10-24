use libc::{free, sleep, ucontext_t, usleep};
use crate::my_pthread::{my_thread_yield, MyPthread, SchedulerEnum};
use crate::my_pthread_pool::{ remove_thread};
use crate::handler::{HANDLER};


pub(crate) fn  my_mutex_init()-> bool {
    let mutex = false;
    return mutex
}

pub(crate) fn change_mutex_state(mut mutex: bool) -> bool {
    match mutex {
        true => {
            return false;
        }
        false => {
            return true;
        }
    }
}

pub(crate) unsafe fn my_mutex_lock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = true;
    return (handler);
}

pub(crate) unsafe fn my_mutex_destroy(mut handler: HANDLER) -> HANDLER {
    handler.mutex = false;
    return handler;
}

//my_mutex_unlock
pub(crate) unsafe fn my_mutex_unlock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = false;
    return handler;
}

//my_mutex_trylock
pub(crate) unsafe fn my_mutex_trylock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = true;
    return handler;
}
