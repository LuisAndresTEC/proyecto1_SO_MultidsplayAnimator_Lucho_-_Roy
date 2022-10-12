use std::ptr::null;
use libc::{clone, sched_param, ucontext_t};
use crate::mutex::{MyMutex,my_mutex_init};
use crate::my_pthread::{MyPthread, schedulerEnum, states};

#[derive(Clone)]
pub(crate) struct PthreadPool {
    pub(crate) scheduler: schedulerEnum,
    //pub(crate) pthreads: Vec<MyPthread>,
    pub(crate) rr_pthreads: Vec<MyPthread>,
    pub(crate) lt_pthreads: Vec<MyPthread>,
    pub(crate) rt_pthreads: Vec<MyPthread>,
    pub(crate) actual_thread: Vec<MyPthread>,
    pub(crate) mutex: MyMutex,
    pub(crate) serial: u32
}impl PthreadPool {
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
    pub(crate) fn get_index_by_id(&self, id: u32) -> Option<usize> {
        for (i, pthread) in self.rr_pthreads.iter().enumerate() {
            if pthread.id == id {
                return Some(i);
            }
        }
        for (i, pthread) in self.lt_pthreads.iter().enumerate() {
            if pthread.id == id {
                return Some(i);
            }
        }
        for (i, pthread) in self.rt_pthreads.iter().enumerate() {
            if pthread.id == id {
                return Some(i);
            }
        }
        None
    }
    //metodo que me dice la cantidad de threads en un determinado estado en una de las listas del pool
    pub(crate) fn get_count_by_state(&self, state: states, sched: schedulerEnum) -> u32 {
        let mut count = 0;
        match sched {
            schedulerEnum::round_robin => {
                for pthread in &self.rr_pthreads {
                    if pthread.state == state {
                        count += 1;
                    }
                }
            }
            schedulerEnum::lottery => {
                for pthread in &self.lt_pthreads {
                    if pthread.state == state {
                        count += 1;
                    }
                }
            }
            schedulerEnum::real_time => {
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
}



pub(crate) fn create_pthread_pool() -> PthreadPool {
    let mut pool = PthreadPool {
        scheduler: schedulerEnum::round_robin,
        //pthreads: Vec::new(),
        rr_pthreads: Vec::new(),
        lt_pthreads: Vec::new(),
        rt_pthreads: Vec::new(),
        actual_thread: Vec::new(),
        mutex: my_mutex_init(),
        serial: 0
    };
    return pool
}

pub(crate) fn remove_thread(mut pool: PthreadPool, thread_id: usize) -> PthreadPool {
    let mut thread = pool.get_by_id(thread_id as u32).unwrap().clone();
    match thread.sched {
        schedulerEnum::round_robin => {
            if pool.rr_pthreads[pool.get_index_by_id(thread_id as u32).unwrap()].id == thread_id as u32 {
                pool.rr_pthreads.remove(pool.get_index_by_id(thread_id as u32).unwrap());
            }
        }
        schedulerEnum::lottery => {
            if pool.lt_pthreads[pool.get_index_by_id(thread_id as u32).unwrap()].id == thread_id as u32 {
                pool.lt_pthreads.remove(pool.get_index_by_id(thread_id as u32).unwrap());
            }
        }
        schedulerEnum::real_time => {
            if pool.rt_pthreads[pool.get_index_by_id(thread_id as u32).unwrap()].id == thread_id as u32 {
                pool.rt_pthreads.remove(pool.get_index_by_id(thread_id as u32).unwrap());
            }
        }
    }
    return pool
}


pub(crate) fn change_scheduler(mut pool: PthreadPool, scheduler: schedulerEnum) -> PthreadPool {
    pool.scheduler = scheduler;
    return pool
}