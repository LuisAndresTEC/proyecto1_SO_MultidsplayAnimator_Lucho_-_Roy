use std::mem;
use std::ptr::null;
use libc::{clone, sched_param, ucontext_t};
use crate::handler::HANDLER;
use crate::mutex::{my_mutex_init};
use crate::my_pthread::{MyPthread, SchedulerEnum, states};

#[derive(Clone)]
pub(crate) struct PtheadPool {
    //pub(crate) pthreads: Vec<MyPthread>,
    pub(crate) rr_pthreads: Vec<MyPthread>,
    pub(crate) lt_pthreads: Vec<MyPthread>,
    pub(crate) rt_pthreads: Vec<MyPthread>,
    pub(crate) actual_thread: Vec<MyPthread>,
    pub(crate) actual_context: Vec<Option<ucontext_t>>,
    pub(crate) rr_contexts: Vec<Option<ucontext_t>>,
    pub(crate) lt_contexts: Vec<Option<ucontext_t>>,
    pub(crate) rt_contexts: Vec<Option<ucontext_t>>,
}impl PtheadPool {
    pub(crate) fn get_by_id(&self, id: u32) -> Option<&MyPthread> {
        for pthread in &self.rr_pthreads {
            if pthread.id == id {
                return Some(pthread);
            }
        }
        for pthread in &self.lt_pthreads {
            if pthread.id == id {
                return Some(pthread);
            }
        }
        for pthread in &self.rt_pthreads {
            if pthread.id == id {
                return Some(pthread);
            }
        }
        None
    }
    pub(crate) fn get_index_by_id(&self, id: u32) -> usize {
        for (i, pthread) in self.rr_pthreads.iter().enumerate() {
            if pthread.id == id {
                return i;
            }
        }
        for (i, pthread) in self.lt_pthreads.iter().enumerate() {
            if pthread.id == id {
                return i;
            }
        }
        for (i, pthread) in self.rt_pthreads.iter().enumerate() {
            if pthread.id == id {
                return i;
            }
        }
        return 200;
    }
    //metodo que me dice la cantidad de threads en un determinado estado en una de las listas del pool
    pub(crate) fn get_count_by_state(&self, state: states, sched: SchedulerEnum) -> u32 {
        let mut count = 0;
        match sched {
            SchedulerEnum::RoundRobin => {
                for pthread in &self.rr_pthreads {
                    if pthread.state == state {
                        count += 1;
                    }
                }
            }
            SchedulerEnum::Lottery => {
                for pthread in &self.lt_pthreads {
                    if pthread.state == state {
                        count += 1;
                    }
                }
            }
            SchedulerEnum::RealTime => {
                for pthread in &self.rt_pthreads {
                    if pthread.state == state {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    //metodo que retorna el indice del thread de mayor prioridad en la lista de rt_pthreads
    pub(crate) fn get_highest_priority_index(&self) -> Option<usize> {
        let mut highest_priority = 0;
        let mut index = 0;
        for (i, pthread) in self.rt_pthreads.iter().enumerate() {
            if pthread.priority > highest_priority {
                highest_priority = pthread.priority;
                index = i;
            }
        }
        Some(index)
    }

    pub(crate) fn get_active_threads_number(&self, sched: SchedulerEnum) -> usize {
        let mut count = 0;
        match sched {
            SchedulerEnum::RoundRobin => {
                for pthread in &self.rr_pthreads {
                    if pthread.state == states::running || pthread.state == states::ready {
                        count += 1;
                    }
                }
            }
            SchedulerEnum::Lottery => {
                for pthread in &self.lt_pthreads {
                    if pthread.state == states::running || pthread.state == states::ready {
                        count += 1;
                    }
                }
            }
            SchedulerEnum::RealTime => {
                for pthread in &self.rt_pthreads {
                    if pthread.state == states::running || pthread.state == states::ready {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }
}
pub(crate) fn state_validation(state: states , thread: MyPthread) -> bool {
    match state {
        states::running => {
            if thread.state == states::ready {
                return true;
            }
        }
        states::ready => {
            if thread.state == states::running {
                return true;
            }
        }
        states::blocked => {
            if thread.state == states::running {
                return true;
            }
        }
        states::terminated => {
            if thread.state == states::running {
                return true;
            }
        }
        _ => {}
    }
    return false;
    }




pub(crate) fn create_pthread_pool() -> PtheadPool {
    let mut pool = PtheadPool {
        rr_pthreads: Vec::new(),
        lt_pthreads: Vec::new(),
        rt_pthreads: Vec::new(),
        actual_thread: Vec::new(),
        actual_context: Vec::new(),
        rr_contexts: Vec::new(),
        lt_contexts: Vec::new(),
        rt_contexts: Vec::new(),
    };
    return pool
}

pub(crate) fn remove_thread(mut handler: HANDLER, mut thread_id: usize) -> HANDLER {
    let mut thread = handler.pthread_pool.get_by_id(thread_id as u32).unwrap().clone();
    match thread.sched {
        SchedulerEnum::RoundRobin => {
            if handler.pthread_pool.rr_pthreads[handler.pthread_pool.get_index_by_id(thread_id as u32)].id == thread_id as u32 {
                handler.pthread_pool.rr_pthreads.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
                handler.pthread_pool.rr_contexts.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
            }
        }
        SchedulerEnum::Lottery => {
            if handler.pthread_pool.lt_pthreads[handler.pthread_pool.get_index_by_id(thread_id as u32)].id == thread_id as u32 {
                handler.pthread_pool.lt_pthreads.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
                handler.pthread_pool.lt_contexts.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
            }
        }
        SchedulerEnum::RealTime => {
            if handler.pthread_pool.rt_pthreads[handler.pthread_pool.get_index_by_id(thread_id as u32)].id == thread_id as u32 {
                handler.pthread_pool.rt_pthreads.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
                handler.pthread_pool.rt_contexts.remove(handler.pthread_pool.get_index_by_id(thread_id as u32));
            }
        }
    }
    return handler;
}


