mod my_pthread;
use crate:: my_pthread::{my_thread_create, my_thread_end, schedulerEnum};
mod my_pthread_pool;
mod my_schedulers;
mod mutex;


use crate::my_pthread_pool::{create_pthread_pool};



extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
}


fn main() {
    let mut pool = create_pthread_pool();
    unsafe { pool = my_thread_create(5, pool, f1, schedulerEnum::round_robin); }
    unsafe { pool = my_thread_create(3, pool, f1, schedulerEnum::round_robin); }
    unsafe { pool = my_thread_create(1, pool, f1, schedulerEnum::real_time); }
    unsafe { pool = my_thread_create(2, pool, f1, schedulerEnum::real_time); }
    unsafe { pool = my_thread_create(4, pool, f1, schedulerEnum::lottery); }
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