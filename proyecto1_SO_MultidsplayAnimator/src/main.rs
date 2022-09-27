mod my_pthread;
use my_pthread::my_pthread_create;



fn main() {
    let threads = my_pthread_create(5);
    let thread = my_pthread_detach(1, threads);

}
