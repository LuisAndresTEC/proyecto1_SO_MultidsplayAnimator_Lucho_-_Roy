use std::any::Any;
use std::fs::copy;
use std::mem;
use std::ops::Index;
use std::ptr::null;
use std::task::Context;
//use crate::add_pthread;
use crate::my_pthread_pool::{PthreadPool, remove_thread};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, remove, clone_args};
//se defien el maximo de threads que se pueden crear
const MAX_THREADS: usize = 4;

//este objeto es thread sobre el cual se va a trabajar
#[derive(Clone, Copy)]
pub(crate) struct MyPthread {
    pub(crate) id: u32,
    pub(crate) state: states,
    pub(crate) priority: u64,
    pub(crate) context: ucontext_t,
    pub(crate) sched: schedulerEnum,
    pub(crate) tickets: Option<usize>
}


//se esblecen los nombres de los diferentes tipos de schedulers
#[derive(Clone, Copy)]
pub(crate) enum schedulerEnum {
    real_time,
    round_robin,
    lottery
}

//se establecen los estados para los threads
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum states {
    running,
    ready,
    blocked,
    terminated,
}


pub(crate) unsafe fn my_thread_create(priority: u64, mut pool: PthreadPool, func: extern "C" fn(), mut scheduler: schedulerEnum) -> PthreadPool {
    //Se establece el contaxt para ese nuevo thread
    unsafe {
        let mut starter: [c_char; 8192] = [mem::zeroed(); 8192];

        let mut contextCreating: ucontext_t = mem::uninitialized();
        getcontext(&mut contextCreating as *mut ucontext_t);
        contextCreating.uc_stack.ss_sp = starter.as_mut_ptr() as *mut c_void;
        contextCreating.uc_stack.ss_size = mem::size_of_val(&starter);

        contextCreating.uc_link = match None {
            Some(ref mut x) => &mut *x,
            //cero porque el panic da un problema en el primer thread
            None => 0 as *mut ucontext_t,
        };

        makecontext(&mut contextCreating as *mut ucontext_t, func, 0);
        //se crea el thread
        let mut thread = MyPthread {
            id: pool.serial,
            state: states::ready,
            priority: priority,
            context: contextCreating,
            sched: scheduler,
            tickets: None
        };
        //se agrega el thread a la pool
        pool.pthreads.push(thread.clone());
        pool.serial += 1;
        match thread.sched {
            schedulerEnum::round_robin => {
                pool.rr_pthreads.push(pool.pthreads[pool.pthreads.len() - 1]);
            }
            schedulerEnum::lottery => {
                pool.lt_pthreads.push(pool.pthreads[pool.pthreads.len() - 1]);
            }
            schedulerEnum::real_time => {
                pool.rt_pthreads.push(pool.pthreads[pool.pthreads.len() - 1]);
            }
        }
    }
    return pool;
}

pub(crate) unsafe fn my_thread_yield(mut pool: PthreadPool) -> PthreadPool {
    let mut thread_update= pool.actual_thread[0];
    match pool.scheduler {
        schedulerEnum::round_robin => {
            if state_validation(states::ready, pool.rr_pthreads[0]){
                thread_update = pool.rr_pthreads[0].clone();
                let mut aux_pthread = pool.rr_pthreads[0];
                pool.rr_pthreads.remove(0);
                pool.rr_pthreads.push(aux_pthread);
            }
        }
        schedulerEnum::lottery => {
            if state_validation(states::ready, pool.lt_pthreads[0]) {
                thread_update = pool.lt_pthreads[0].clone();
                let mut aux_pthread = pool.lt_pthreads[0];
                pool.lt_pthreads.remove(0);
                pool.lt_pthreads.push(aux_pthread);
            }
        }

        schedulerEnum::real_time => {
            if state_validation(states::ready, pool.rt_pthreads[0]) {
                thread_update = pool.rt_pthreads[0].clone();
                let mut aux_pthread = pool.rt_pthreads[0];
                pool.rt_pthreads.remove(0);
                pool.rt_pthreads.push(aux_pthread);
            }
        }
    }
    if thread_update.id == pool.actual_thread[0].id {
        panic!("No hay contextos disponibes");
    }else{
        swapcontext(&mut pool.actual_thread[0].context as *mut ucontext_t, &mut thread_update.context as *mut ucontext_t);
        pool.actual_thread[0] = thread_update.clone();
    }
    return pool;
}


pub(crate) fn my_thread_detach(mut pool: PthreadPool) -> PthreadPool {
    pool.rr_pthreads.remove(0);
    pool.lt_pthreads.remove(0);
    pool.rt_pthreads.remove(0);
    pool = remove_thread(pool,0);
    return pool;
}

//esta funcion espera a que la ejecucion de un thread termine
pub(crate) fn my_thread_join(mut pool: PthreadPool) -> PthreadPool {
    let mut thread_update= pool.actual_thread[0];
    match pool.scheduler {
        schedulerEnum::round_robin => {
            if state_validation(states::ready, pool.rr_pthreads[0]) {
                thread_update = pool.rr_pthreads[0];
            }
        }
        schedulerEnum::lottery => {
            if state_validation(states::ready, pool.rr_pthreads[0]) {
                thread_update = pool.lt_pthreads[0];
            }
        }
        schedulerEnum::real_time => {
            if state_validation(states::ready, pool.rr_pthreads[0]) {
                thread_update = pool.rt_pthreads[0];
            }
        }
    }
    if thread_update.id == pool.actual_thread[0].id {
        panic!("No hay contextos disponibes");
    }else{
        thread_update.state = states::blocked;
        pool.actual_thread[0] = thread_update.clone();
        return pool;
    }

}


pub(crate) fn state_validation(state: states, thread: MyPthread) -> bool {
    if thread.state == state {
        return true;
    }else {
        return false;
    }
}

pub(crate) fn my_thread_end(mut pool: PthreadPool, index: usize) -> PthreadPool {
    pool = remove_thread(pool, index);
    return pool
}


pub(crate) fn my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread {
    match scheduler {
        0 => thread.sched = schedulerEnum::real_time,
        1 => thread.sched = schedulerEnum::round_robin,
        2 => thread.sched = schedulerEnum::lottery,
        _ => thread.sched = schedulerEnum::round_robin
    }
    return thread
}

pub(crate) fn my_thread_state(mut thread: MyPthread, state: u32)-> MyPthread{
    match state {
        0 => thread.state = states::running,
        1 => thread.state = states::ready,
        2 => thread.state = states::blocked,
        3 => thread.state = states::terminated,
        _ => thread.state = states::ready
    }
    return thread
}
