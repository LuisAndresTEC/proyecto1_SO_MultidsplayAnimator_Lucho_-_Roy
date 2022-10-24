use std::os::raw::c_uint;
use crate::my_pthread::{my_thread_detach, /*my_thread_join,*/ my_thread_yield, MyPthread, states};
use crate::my_pthread_pool::{ remove_thread};
use crate::SchedulerEnum;
use rand::Rng;
use std::time::Duration;
use libc::{nanosleep, sleep};
use crate::handler::HANDLER;
use crate::mutex::my_mutex_unlock;
use crate::SchedulerEnum::RoundRobin;


//esta funcion no retorna nada, solo cambia el estado del hilo actual
pub(crate) unsafe fn scheduler_round_robin(mut handler: HANDLER) -> HANDLER {
    let quantum: i32 = 0.05 as i32;
    while handler.pthread_pool.get_active_threads_number(RoundRobin)> 0 {
        handler = my_mutex_unlock(handler);
        handler = my_thread_yield(handler);
        sleep(quantum as c_uint);
        handler.pthread_pool.actual_thread[0].finishing_validator();
    }
    return handler;

}


pub(crate) fn shortest_job_selector (mut handler: HANDLER) -> MyPthread {
    let mut shortest_job = handler.pthread_pool.rt_pthreads[0].clone();
    for pthread in handler.pthread_pool.rt_pthreads {
        if pthread.context.uc_stack.ss_size < shortest_job.context.uc_stack.ss_size && (pthread.state == states::ready || pthread.state == states::running) {
            shortest_job = pthread.clone();
        }
    }
    return shortest_job;
}

//implementar el monotonic pag.521
//EDF
pub(crate) unsafe fn scheduler_real_time(mut handler: HANDLER) -> HANDLER {
    let quantum: i32 = 0.05 as i32;
    while handler.pthread_pool.get_active_threads_number(SchedulerEnum::RealTime)> 0 {
        let mut next_thread = shortest_job_selector(handler.clone());
        handler = my_mutex_unlock(handler.clone());
        //handler.pthread_pool = my_thread_join(handler.pthread_pool.clone() , handler.pthread_pool.get_index_by_id(next_thread.id).unwrap());
        sleep(quantum as c_uint);
        handler.pthread_pool.actual_thread[0].finishing_validator();
    }
    return handler;

}



pub(crate) unsafe fn scheduler_lottery(mut handler: HANDLER) -> HANDLER {
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
    for thread in &handler.pthread_pool.lt_pthreads {
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
    let mut next_thread = handler.pthread_pool.get_by_id(winner.thread_id).unwrap().clone();
    tombola.tickets.remove(indice_ticket);
    if next_thread.state == states::ready {
        handler = my_mutex_unlock(handler);
        //handler.pthread_pool = my_thread_join(handler.pthread_pool.clone() , handler.pthread_pool.get_index_by_id(next_thread.id).unwrap());
        handler.pthread_pool.actual_thread[0].finishing_validator();
    }
    return handler;
}




