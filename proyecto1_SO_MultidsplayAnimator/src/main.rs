mod my_pthread;
use my_pthread::my_thread_create;
mod my_pthread_pool;
use crate::my_pthread_pool::create_pthread_pool;
use my_pthread_pool::add_pthread;


fn main() {
    let mut pool = create_pthread_pool();
    pool = my_thread_create(5, pool);

}
