use libc::{free, sleep, ucontext_t, usleep};
use crate::my_pthread::{my_thread_yield, MyPthread, SchedulerEnum};
use crate::my_pthread_pool::{ remove_thread};
use crate::handler::{HANDLER};

//función que inicializa el mutex
pub(crate) fn  my_mutex_init()-> bool {
    let mutex = false;
    return mutex
}

//funcion que cambia el estado del mutex
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

//funcion que bloquea el mutex
pub(crate) unsafe fn my_mutex_lock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = true;
    handler
}

//funcion que destruye el mutex
pub(crate) unsafe fn my_mutex_destroy(mut handler: HANDLER) -> HANDLER {
    handler.mutex = false;
    return handler;
}

//funcion que desbloquea el mutex
pub(crate) unsafe fn my_mutex_unlock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = false;
    return handler;
}

//funcion que intenta bloquear el mutex
pub(crate) unsafe fn my_mutex_trylock(mut handler: HANDLER) -> HANDLER {
    handler.mutex = true;
    return handler;
}
