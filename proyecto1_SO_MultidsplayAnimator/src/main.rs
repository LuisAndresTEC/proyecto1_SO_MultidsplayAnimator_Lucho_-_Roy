mod my_pthread;
use my_pthread::my_thread_create;
mod my_pthread_pool;
mod my_schedulers;

use crate::my_pthread_pool::create_pthread_pool;
use my_pthread_pool::add_pthread;


extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
}


fn main() {
    let mut pool = create_pthread_pool();
    unsafe { pool = my_thread_create(5, pool, f1); }
}