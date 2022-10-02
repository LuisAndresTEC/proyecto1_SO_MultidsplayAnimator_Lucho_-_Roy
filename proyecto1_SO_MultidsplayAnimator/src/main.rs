mod my_pthread;
use my_pthread::my_thread_create;
mod my_pthread_pool;
mod my_schedulers;

use crate::my_pthread_pool::create_pthread_pool;
use my_pthread_pool::add_pthread;
use crate::my_pthread::my_thread_end;


extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
}


fn main() {
    let mut pool = create_pthread_pool();
    unsafe { pool = my_thread_create(5, pool, f1); }
    unsafe { pool = my_thread_create(3, pool, f1); }
    unsafe { pool = my_thread_create(1, pool, f1); }
    unsafe { pool = my_thread_create(2, pool, f1); }
    unsafe { pool = my_thread_create(4, pool, f1); }
    //imprime el len de los treads en el pool
    println!("{}", pool.pthreads.len().to_string());
    for i in 0..pool.pthreads.len() {
        println!("\t\tThread {} priority {}\n\n", pool.pthreads[i].id, pool.pthreads[i].priority);
        //imprime informacion sobrelos contextos
    }
    //Elimina un thread del pool
    pool = my_thread_end(pool, 0);
    println!("---------------------Ya se eliminó el thread 0---------------------");
    //imprime el len de los treads en el pool
    println!("{}", pool.pthreads.len().to_string());
    for i in 0..pool.pthreads.len() {
        println!("\t\tThread {} priority {}\n\n", pool.pthreads[i].id, pool.pthreads[i].priority);
        //imprime informacion sobrelos contextos
    }
}