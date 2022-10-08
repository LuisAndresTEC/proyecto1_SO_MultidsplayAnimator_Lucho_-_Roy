use std::any::Any;
use std::fs::copy;
use std::mem;
use std::ops::Index;
use std::ptr::null;
use std::task::Context;
use crate::add_pthread;
use crate::my_pthread_pool::{PthreadPool, remove_thread};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, remove, clone_args};
//se defien el maximo de threads que se pueden crear
const MAX_THREADS: usize = 4;

//este objeto es thread sobre el cual se va a trabajar
pub(crate) struct MyPthread {
    pub(crate) id: u32,
    pub(crate) state: states,
    pub(crate) priority: u64,
    pub(crate) context: ucontext_t,
    pub(crate) sched: schedulerEnum,
    pub(crate) tickets: Option<usize>
}


//se esblecen los nombres de los diferentes tipos de schedulers
pub(crate) enum schedulerEnum {
    real_time,
    round_robin,
    lottery
}

//se establecen los estados para los threads
pub(crate) enum states {
    running,
    ready,
    blocked,
    terminated,
}


pub(crate) unsafe fn my_thread_create(priority: u64, mut pool: PthreadPool, func: extern "C" fn(), mut scheduler: schedulerEnum) -> PthreadPool {

    unsafe {
        let mut starter: [c_char; 8192] = [mem::zeroed(); 8192];

        let mut contextCreating: ucontext_t = mem::uninitialized();
        getcontext(&mut contextCreating as *mut ucontext_t);
        contextCreating.uc_stack.ss_sp = starter.as_mut_ptr() as *mut c_void;
        contextCreating.uc_stack.ss_size = mem::size_of_val(&starter);
        contextCreating.uc_link = match pool.actual_context {
            Some(ref mut x) => &mut *x,
            //cero porque el panic da un problema en el primer thread
            None => 0 as *mut ucontext_t,
        };
        makecontext(&mut contextCreating as *mut ucontext_t, func, 0);


    let mut thread = MyPthread {
        id: pool.serial,
        state: states::ready,
        priority: priority,
        context: contextCreating,
        sched: scheduler,
        tickets: None
    };

        pool = add_pthread(pool, thread);
        return pool
    }
}

/*
pub(crate) fn my_thread_yield(mut pool: PthreadPool, scheduler: schedulerEnum) -> PthreadPool {
    let mut aux = Vec::new();
    match  scheduler{
        schedulerEnum::round_robin => unsafe {
            aux = pool.rr_pthreads.unwrap();
            //swapcontext(aux[0].context as *mut ucontext_t, aux[1].context as *mut ucontext_t);
            aux.rotate_left(0);
            aux.remove(0);
            pool.rr_pthreads = Some(aux);
        }
        schedulerEnum::lottery => unsafe {
            aux = pool.lt_pthreads.unwrap();
            //swapcontext(aux[0].context as *mut ucontext_t, aux[1].context as *mut ucontext_t);

        }
        schedulerEnum::real_time => unsafe {
            aux = pool.rt_pthreads.unwrap();
            // swapcontext(aux[0].context as *mut ucontext_t, aux[1].context as *mut ucontext_t);

        }
    }

    return pool;
}
*/

pub(crate) fn my_thread_end(mut pool: PthreadPool, index: usize) -> PthreadPool {
    pool = remove_thread(pool, index);
    return pool

}



//ESta funcion inicializa un hilo especifico segun su prioridad y los asigna a los campos de transito en el pool


pub(crate) fn my_thread_join(thread: MyPthread) -> MyPthread {
    let mut thread = thread;
    thread.state = states::blocked;
    return thread
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
