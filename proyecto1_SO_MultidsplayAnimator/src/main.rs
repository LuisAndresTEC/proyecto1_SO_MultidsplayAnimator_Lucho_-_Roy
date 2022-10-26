#[path = "pthread/my_pthread.rs"]
pub(crate) mod my_pthread;

use libc::{getcontext, setcontext, swapcontext};
use crate:: my_pthread::{my_thread_create,  SchedulerEnum};
use crate::handler::create_handler;
use crate::my_pthread::{CURRENT_THREAD, EXIT_CONTEXT, my_thread_end};

#[path = "pthread/my_pthread_pool.rs"]
pub(crate) mod my_pthread_pool;
#[path = "pthread/my_schedulers.rs"]
pub(crate) mod my_schedulers;
#[path = "pthread/mutex.rs"] mod mutex;
#[path = "parse/parser.rs"] mod parser;
#[path = "pthread/handler.rs"] mod handler;


use crate::my_pthread_pool::{create_pthread_pool};
use crate::my_schedulers::scheduler_round_robin;
use crate::parser::{load_file, set_values};


extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f2() {
    println!("INICIO 2");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(1), rb_thread::child_match(2));}
    println!("FIN 2");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f3() {
    println!("INICIO 3");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 3");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f4() {
    println!("INICIO 4");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 4");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f5() {
    println!("INICIO 5");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 5");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f6() {
    println!("INICIO 6");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 6");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f7() {
    println!("INICIO 7");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 7");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f8() {
    println!("INICIO 8");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 8");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f9() {
    println!("INICIO 9");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 9");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}

extern "C" fn f10() {
    println!("INICIO 10");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 10");
    unsafe{
        swapcontext(CURRENT_THREAD, EXIT_CONTEXT);}
}


fn main() {
    unsafe {
        let mut file = load_file();
        let mut language = set_values(file);
        let mut handler = unsafe { create_handler() };


        unsafe { handler = my_thread_create(3, handler, f1, SchedulerEnum::RoundRobin); }
        unsafe { handler = my_thread_create(7, handler, f2, SchedulerEnum::RoundRobin); }
        unsafe { handler = my_thread_create(1, handler, f3, SchedulerEnum::RoundRobin); }
        unsafe { handler = my_thread_create(2, handler, f4, SchedulerEnum::RoundRobin); }
        unsafe { handler = my_thread_create(2, handler, f5, SchedulerEnum::RealTime); }
        unsafe { handler = my_thread_create(3, handler, f6, SchedulerEnum::RealTime); }
        unsafe { handler = my_thread_create(2, handler, f7, SchedulerEnum::RealTime); }
        unsafe { handler = my_thread_create(4, handler, f8, SchedulerEnum::Lottery); }
        unsafe { handler = my_thread_create(8, handler, f9, SchedulerEnum::Lottery); }
        unsafe { handler = my_thread_create(5, handler, f10, SchedulerEnum::Lottery); }
        if handler.pthread_pool.actual_thread.is_empty() {
            match handler.scheduler {
                SchedulerEnum::RoundRobin => {
                    handler.pthread_pool.actual_thread.push(handler.pthread_pool.rr_pthreads.last().unwrap().clone());
                    handler.pthread_pool.actual_context.push(handler.pthread_pool.rr_contexts.last().unwrap().clone());
                    // handler.pthread_pool.rr_pthreads.pop();
                    //handler.pthread_pool.rr_contexts.pop();
                }
                SchedulerEnum::RealTime => {
                    handler.pthread_pool.actual_thread.push(handler.pthread_pool.rt_pthreads.last().unwrap().clone());
                    handler.pthread_pool.actual_context.push(handler.pthread_pool.rt_contexts.last().unwrap().clone());
                    //handler.pthread_pool.rt_pthreads.pop();
                    //handler.pthread_pool.rt_contexts.pop();
                }
                SchedulerEnum::Lottery => {
                    handler.pthread_pool.actual_thread.push(handler.pthread_pool.lt_pthreads.last().unwrap().clone());
                    handler.pthread_pool.actual_context.push(handler.pthread_pool.lt_contexts.last().unwrap().clone());
                    //handler.pthread_pool.lt_pthreads.pop();
                    //handler.pthread_pool.lt_contexts.pop();
                }
            }
        }
        handler.__run_threads__();
    }
}


