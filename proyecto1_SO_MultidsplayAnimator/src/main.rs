#[path = "pthread/my_pthread.rs"] mod my_pthread;
#[path = "pthread/my_pthread_pool.rs"] mod my_pthread_pool;
#[path = "pthread/my_schedulers.rs"] mod my_schedulers;
#[path = "pthread/mutex.rs"] mod mutex;
#[path = "parse/parser.rs"] mod parser;
#[path = "pthread/handler.rs"] mod handler;
#[path = "animator/my_canvas.rs"] mod my_canvas;
use crate:: my_pthread::{my_thread_create,  SchedulerEnum, INITIAL_CONTEXT, FINAL_CONTEXT, my_thread_end};
use crate::handler::create_handler;
use crate::my_pthread_pool::{create_pthread_pool};
use crate::my_schedulers::scheduler_round_robin;
use crate::parser::{load_file, set_values};
use libc::{getcontext, setcontext, swapcontext};
use crate::my_canvas::{animation};
use crate::SchedulerEnum::{Lottery, RealTime, RoundRobin};

static mut file: Vec<String> = Vec::new();



/*
extern "C"  fn animator() {
    unsafe {
        animation(file.clone());
        swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);
    }
}
*/
extern "C" fn f2() {
    println!("INICIO 2");
    println!("FIN 2");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f3() {
    println!("INICIO 3");
    println!("FIN 3");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f4() {
    println!("INICIO 4");
    println!("FIN 4");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f5() {
    println!("INICIO 5");
    println!("FIN 5");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f6() {
    println!("INICIO 6");
    println!("FIN 6");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f7() {
    println!("INICIO 7");
    println!("FIN 7");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f8() {
    println!("INICIO 8");
    println!("FIN 8");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f9() {
    println!("INICIO 9");
    println!("FIN 9");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}

extern "C" fn f10() {
    println!("INICIO 10");
    println!("FIN 10");
    unsafe{ swapcontext(INITIAL_CONTEXT, FINAL_CONTEXT);}
}



fn main() {
    unsafe {
        let args = std::env::args().collect::<Vec<String>>();
        if args[1] != "animar" || args[2] != "-c"{
            panic!("Error en los argumentos");
        }
        let mut config_path = args[3].clone();

        file = load_file(config_path);
        let mut language = set_values(file.clone());
        //animator();
        let mut handler = unsafe { create_handler(RealTime) };



        /*unsafe { handler = my_thread_create(3, handler, animator, SchedulerEnum::RealTime); }
        unsafe { handler = my_thread_create(6, handler, animator, SchedulerEnum::RealTime); }*/
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

                }
                SchedulerEnum::RealTime => {
                    handler.pthread_pool.actual_thread.push(handler.pthread_pool.rt_pthreads.last().unwrap().clone());
                    handler.pthread_pool.actual_context.push(handler.pthread_pool.rt_contexts.last().unwrap().clone());

                }
                SchedulerEnum::Lottery => {
                    handler.pthread_pool.actual_thread.push(handler.pthread_pool.lt_pthreads.last().unwrap().clone());
                    handler.pthread_pool.actual_context.push(handler.pthread_pool.lt_contexts.last().unwrap().clone());
                }
                _ => {}
            }
        }
        handler.__run_threads__();
    }
}


