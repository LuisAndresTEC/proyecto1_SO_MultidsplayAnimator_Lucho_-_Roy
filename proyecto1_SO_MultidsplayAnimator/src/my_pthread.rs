use std::fs::copy;
use std::ptr::null;
use std::task::Context;

//se defien el maximo de threads que se pueden crear
const MAX_THREADS: usize = 4;

//este objeto es thread sobre el cual se va a trabajar
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
}

//se establecen los estados para los threads
enum states {
    running,
    ready,
    blocked,
    terminated,
}

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

}