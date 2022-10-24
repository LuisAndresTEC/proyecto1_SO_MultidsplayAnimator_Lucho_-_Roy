#[path = "pthread/my_pthread.rs"]
pub(crate) mod my_pthread;

use libc::{getcontext, setcontext};
use crate:: my_pthread::{my_thread_create,  SchedulerEnum};
use crate::handler::create_handler;
use crate::my_pthread::my_thread_end;

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
}

extern "C" fn f2() {
    println!("INICIO 2");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(1), rb_thread::child_match(2));}
    println!("FIN 2");
}

extern "C" fn f3() {
    println!("INICIO 3");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 3");
}

extern "C" fn f4() {
    println!("INICIO 4");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 4");
}

extern "C" fn f5() {
    println!("INICIO 5");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 5");
}

extern "C" fn f6() {
    println!("INICIO 6");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 6");
}

extern "C" fn f7() {
    println!("INICIO 7");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 7");
}

extern "C" fn f8() {
    println!("INICIO 8");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 8");
}

extern "C" fn f9() {
    println!("INICIO 9");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 9");
}

extern "C" fn f10() {
    println!("INICIO 10");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(2), rb_thread::child_match(0));}
    println!("FIN 10");
}


fn main() {
    let mut file = load_file();
    let mut language = set_values(file);
    let mut handler = unsafe { create_handler() };
    unsafe { handler = my_thread_create(5, handler, f1, SchedulerEnum::RoundRobin); }
    unsafe { handler = my_thread_create(3, handler, f2, SchedulerEnum::RoundRobin); }
    unsafe { handler = my_thread_create(1, handler, f3, SchedulerEnum::RoundRobin); }
    unsafe { handler = my_thread_create(2, handler, f4, SchedulerEnum::RoundRobin); }
    unsafe { handler = my_thread_create(2, handler, f5, SchedulerEnum::RealTime); }
    unsafe { handler = my_thread_create(2, handler, f6, SchedulerEnum::RealTime); }
    unsafe { handler = my_thread_create(4, handler, f7, SchedulerEnum::Lottery); }
    unsafe { handler = my_thread_create(8, handler, f8, SchedulerEnum::Lottery); }
    unsafe { handler = my_thread_create(5, handler, f9, SchedulerEnum::Lottery); }
    unsafe { handler = my_thread_create(6, handler, f10, SchedulerEnum::Lottery); }
    handler.__run_threads__();



/*
    //imprime el len de los treads en el pool

    println!("\n\nRound Robin Threads");
    for i in 0..handler.pthread_pool.rr_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", handler.rr_pthreads[i].id, handler.rr_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    println!("\n\nReal Time Threads");
    for i in 0..handler.rt_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", handler.rt_pthreads[i].id, handler.rt_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    println!("\n\nLottery Threads");
    for i in 0..handler.lt_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", handler.lt_pthreads[i].id, handler.lt_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    //Elimina un thread del pool
    handler.pthread_pool = my_thread_end(handler.pthread_pool, 0);
    println!("---------------------Ya se eliminó el thread 0---------------------");
    //imprime el len de los treads en el pool
    println!("\n\nRound Robin Threads");
    for i in 0..handler.rr_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", handler.rr_pthreads[i].id, handler.rr_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }*/

}





