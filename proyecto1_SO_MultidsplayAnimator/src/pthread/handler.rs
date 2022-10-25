use std::mem;
use libc::ucontext_t;

use crate::my_pthread_pool::{create_pthread_pool};
use crate::my_schedulers::{scheduler_lottery, scheduler_real_time};
use crate::mutex::{my_mutex_init};
use crate::my_pthread::{SchedulerEnum};
use crate::{my_pthread_pool, scheduler_round_robin};


static mut PARENT: Option<ucontext_t> = None;
static mut THREADS_CONTEXT:Vec<Option<ucontext_t>> = Vec::new();

#[derive(Clone)]
pub(crate) struct HANDLER{
    pub(crate) pthread_pool: my_pthread_pool::PthreadPool,
    pub(crate) scheduler: SchedulerEnum,
    pub(crate) mutex: bool,
    pub(crate) serial: u32,
    //pub(crate) origin_context: Option<ucontext_t>,
}impl HANDLER  {
    pub(crate) fn __run_threads__(mut self) {
        match self.scheduler {
            SchedulerEnum::RoundRobin => unsafe {
                self = scheduler_round_robin(self);
            }
            SchedulerEnum::Lottery => unsafe {
                self = scheduler_lottery(self);
            }
            SchedulerEnum::RealTime => unsafe {
                self = scheduler_real_time(self);
            }
        }
    }

}

pub(crate) unsafe fn create_handler() -> HANDLER {
    unsafe{PARENT = Some(mem::uninitialized());}
    let handler = HANDLER {
        pthread_pool: create_pthread_pool(),
        scheduler: SchedulerEnum::RoundRobin,
        mutex: my_mutex_init(),
        serial: 0,
        //origin_context: mem::uninitialized(),
    };
    return handler;
}

pub(crate) unsafe fn origin_match() -> &'static mut ucontext_t {
    match PARENT {
        Some(ref mut x) => &mut *x,
        None => panic!("No hay contexto de origen"),
    }
}

pub(crate) fn secondary_match(i:usize, handler: HANDLER) -> &'static mut ucontext_t {
    match handler.scheduler {
        SchedulerEnum::RoundRobin => unsafe {
            THREADS_CONTEXT = handler.pthread_pool.rr_contexts.clone();
            match THREADS_CONTEXT[i] {
                Some(ref mut x) => &mut *x,
                None => panic!("No hay contexto de origen"),
            }
        }
        SchedulerEnum::Lottery => unsafe {
            THREADS_CONTEXT = handler.pthread_pool.lt_contexts.clone();
            match THREADS_CONTEXT[i] {
                Some(ref mut x) => &mut *x,
                None => panic!("No hay contexto de origen"),
            }
        }
        SchedulerEnum::RealTime => unsafe {
            THREADS_CONTEXT = handler.pthread_pool.rt_contexts.clone();
            match THREADS_CONTEXT[i] {
                Some(ref mut x) => &mut *x,
                None => panic!("No hay contexto de origen"),
            }
        }
    }
}