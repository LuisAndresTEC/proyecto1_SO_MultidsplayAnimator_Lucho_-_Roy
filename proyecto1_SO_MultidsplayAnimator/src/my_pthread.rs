<<<<<<< HEAD
use std::any::Any;
use std::fs::copy;
use std::ops::Index;
use std::ptr::null;
use std::task::Context;
use crate::add_pthread;
use crate::my_pthread_pool::PthreadPool;
//use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};


=======
use std::fs::copy;
use std::ptr::null;
use std::task::Context;
>>>>>>> origin/main

//se defien el maximo de threads que se pueden crear
const MAX_THREADS: usize = 4;

//este objeto es thread sobre el cual se va a trabajar
<<<<<<< HEAD
pub(crate) struct MyPthread {
    pub(crate) id: u32,
    pub(crate) state: states,
    pub(crate) priority: u64,
    pub(crate) stack: Vec<u8>,
    pub(crate) sched: Scheduler,
}


//se esblecen los nombres de los diferentes tipos de schedulers
enum Scheduler{
    real_time,
    round_robin,
    lottery
=======
pub struct MyPthread {
    id: u32,
    name: String,
    state: ThreadState,
    priority: u32,
    stack: Vec<u8>
}




//estados de los threads
pub struct ThreadState {
    //estado del thread
    //running, ready, blocked, terminated
    state: states,
>>>>>>> origin/main
}

//se establecen los estados para los threads
enum states {
    running,
    ready,
    blocked,
    terminated,
}

<<<<<<< HEAD
pub(crate) fn my_thread_create(priority: u64, mut pool: PthreadPool) -> PthreadPool {
    let mut thread = MyPthread {
        id: pool.serial,
        state: states::ready,
        priority: priority,
        stack: Vec::new(),
        sched: Scheduler::round_robin,
        };
    pool = add_pthread(pool, thread);
    return pool
}

pub(crate) fn my_thread_end(thread: MyPthread) -> MyPthread {
    let mut thread = thread;
    thread.state = states::terminated;
    return thread
}

pub(crate) fn my_thread_yield(thread: MyPthread) -> MyPthread {
    let mut thread = thread;
    let mut stack = thread.stack;
    let last = stack.len() - 1;
    stack.insert(last, *stack.index(0));
    stack.remove(0);
    thread.stack = stack;
    return thread
}

pub(crate) fn my_thread_join(thread: MyPthread) -> MyPthread {
    let mut thread = thread;
    thread.state = states::blocked;
    return thread
}

pub(crate) fn my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread {
    match scheduler {
        0 => thread.sched = Scheduler::real_time,
        1 => thread.sched = Scheduler::round_robin,
        2 => thread.sched = Scheduler::lottery,
        _ => thread.sched = Scheduler::round_robin
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
=======
//se crea un nuevo thread
pub fn my_pthread_create(numberThreads: i32) -> Vec<MyPthread> {
    let mut threads_pool = Vec::new();
    for i in 0..numberThreads {
        if threads_pool.len() > MAX_THREADS {
            println!("No se pueden crear mas threads");
            break;
        }else {
        let thread = MyPthread {
            id: i as u32,
            name: String::from("thread"),
            state: ThreadState {
                state: states::ready
            },
            priority: 1,
            stack: Vec::new(),
        };
        threads_pool.push(thread);
        }
    }
    return threads_pool;
}

pub fn my_pthread_end(threadId: i32, threadPool: Vec<MyPthread>){
    let mut threads_pool = threadPool;
    for i in 0..threads_pool.len() {
        if threads_pool[i].id == threadId as u32 {
            threads_pool[i].state.state = states::terminated;
        }
    }

>>>>>>> origin/main
}