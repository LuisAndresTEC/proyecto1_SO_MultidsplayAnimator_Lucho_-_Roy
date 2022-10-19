#[path = "pthread/my_pthread.rs"]mod my_pthread;

use libc::{getcontext, setcontext};
use crate:: my_pthread::{my_thread_create, my_thread_end, SchedulerEnum};
#[path = "pthread/my_pthread_pool.rs"]mod my_pthread_pool;
#[path = "pthread/my_schedulers.rs"] mod my_schedulers;
#[path = "pthread/mutex.rs"] mod mutex;
#[path = "parse/parser.rs"] mod parser;


use crate::my_pthread_pool::{create_pthread_pool};
use crate::my_schedulers::scheduler_round_robin;
use crate::parser::{load_file, set_values};

/*
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


fn main() {
    let mut pool = create_pthread_pool();
    unsafe { pool = my_thread_create(5, pool, f1, SchedulerEnum::RoundRobin); }
    unsafe { pool = my_thread_create(3, pool, f2, SchedulerEnum::RoundRobin); }
    unsafe { pool = my_thread_create(1, pool, f3, SchedulerEnum::RealTime); }
    unsafe { pool = my_thread_create(2, pool, f4, SchedulerEnum::RealTime); }
    unsafe { pool = my_thread_create(4, pool, f5, SchedulerEnum::Lottery); }
    unsafe { pool = my_thread_create(8, pool, f6, SchedulerEnum::Lottery); }
    unsafe { pool = my_thread_create(5, pool, f7, SchedulerEnum::Lottery); }
    unsafe { pool = my_thread_create(6, pool, f8, SchedulerEnum::Lottery); }
    //pool.actual_thread = Vec::new();
    //pool.actual_thread.push(pool.rr_pthreads[pool.rr_pthreads.len()-1].clone());
    //unsafe { getcontext(&mut pool.actual_thread[0].context); }
    //unsafe { pool = scheduler_round_robin(pool); }



    //imprime el len de los treads en el pool

    println!("\n\nRound Robin Threads");
    for i in 0..pool.rr_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", pool.rr_pthreads[i].id, pool.rr_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    println!("\n\nReal Time Threads");
    for i in 0..pool.rt_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", pool.rt_pthreads[i].id, pool.rt_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    println!("\n\nLottery Threads");
    for i in 0..pool.lt_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", pool.lt_pthreads[i].id, pool.lt_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }
    //Elimina un thread del pool
    pool = my_thread_end(pool, 0);
    println!("---------------------Ya se eliminó el thread 0---------------------");
    //imprime el len de los treads en el pool
    println!("\n\nRound Robin Threads");
    for i in 0..pool.rr_pthreads.len() {

        println!("\t\tThread {} priority {}  \n\n ", pool.rr_pthreads[i].id, pool.rr_pthreads[i].priority,);
        //imprime informacion sobrelos contextos
    }

}
*/

fn main(){
    let mut file = load_file();
    let mut languaje = set_values(file);

}