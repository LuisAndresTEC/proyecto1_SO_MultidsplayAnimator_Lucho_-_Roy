use std::os::raw::c_uint;
use crate::my_pthread::{my_thread_detach, my_thread_join, my_thread_yield, MyPthread, states};
use crate::my_pthread_pool::{PthreadPool, remove_thread};
use crate::SchedulerEnum;
use rand::Rng;
use std::time::Duration;
use libc::{nanosleep, sleep};
use crate::mutex::my_mutex_unlock;
use crate::SchedulerEnum::RoundRobin;


//esta funcion no retorna nada, solo cambia el estado del hilo actual
pub(crate) unsafe fn scheduler_round_robin(mut pool: PthreadPool) -> PthreadPool {
    let quantum: i32 = 0.05 as i32;
    while pool.get_active_threads_number(RoundRobin)> 0 {
        pool = my_mutex_unlock(pool);
        pool = my_thread_yield(pool.clone());
        sleep(quantum as c_uint);
        pool.actual_thread[0].finishing_validator();
    }
    return pool;
}


pub(crate) fn shortest_job_selector (mut pool: PthreadPool) -> MyPthread {
    let mut shortest_job = pool.rt_pthreads[0].clone();
    for pthread in pool.rt_pthreads {
        if pthread.context.uc_stack.ss_size < shortest_job.context.uc_stack.ss_size && (pthread.state == states::ready || pthread.state == states::running) {
            shortest_job = pthread.clone();
        }
    }
    return shortest_job;
}

//implementar el monotonic pag.521
//EDF
pub(crate) unsafe fn scheduler_real_time(mut pool: PthreadPool) -> PthreadPool {
    let quantum: i32 = 0.05 as i32;
    while pool.get_active_threads_number(SchedulerEnum::RealTime)> 0 {
        let mut next_thread = shortest_job_selector(pool.clone());
        pool = my_mutex_unlock(pool.clone());
        pool = my_thread_join(pool.clone() , pool.get_index_by_id(next_thread.id).unwrap());
        sleep(quantum as c_uint);
        pool.actual_thread[0].finishing_validator();
    }
    return pool;
}



pub(crate) unsafe fn scheduler_lottery(mut pool: PthreadPool) -> PthreadPool {
    //se crean los objetos ticket y tombola los cuales se van a utilizar para determinar el hilo a procesar
    #[derive(Clone)]
    pub struct ticket {
        pub ticket_number: u32,
        pub thread_id: u32,
    }
    #[derive(Clone)]
    pub struct tombola{
        pub tickets: Vec<ticket>,
        pub serial: usize,
    }
    //Se inicializa la tombola
    let mut tombola = tombola {
        tickets: Vec::new(),
        serial: 0,
    };

    //se agregan los tickets a la tombola
    for thread in &pool.lt_pthreads {
        for _ in 0..thread.priority {
            tombola.tickets.push(ticket {
                ticket_number: tombola.serial.clone() as u32,
                thread_id: thread.id,
            });
            tombola.serial += 1;
        }
    }

    let mut rng = rand::thread_rng();
    //ciclo
    let indice_ticket = rng.gen_range(0..tombola.tickets.len());
    let mut winner = tombola.tickets[indice_ticket].clone();
    let mut next_thread = pool.get_by_id(winner.thread_id).unwrap().clone();
    tombola.tickets.remove(indice_ticket);
    if next_thread.state == states::ready {
        pool = my_mutex_unlock(pool.clone());
        pool = my_thread_join(pool.clone() , pool.get_index_by_id(next_thread.id).unwrap());
        pool.actual_thread[0].finishing_validator();
    }
    return pool;
}




