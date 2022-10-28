use std::os::raw::c_uint;
use crate::my_pthread::{my_thread_detach, my_thread_join, my_thread_yield, MyPthread, states};
use crate::my_pthread_pool::{ remove_thread};
use crate::SchedulerEnum;
use rand::Rng;
use std::time::Duration;
use libc::{nanosleep, sleep, usleep};
use crate::handler::HANDLER;
use crate::mutex::my_mutex_unlock;
use crate::SchedulerEnum::RoundRobin;


//esta funcion no retorna nada, solo cambia el estado del hilo actual
pub(crate) unsafe fn scheduler_round_robin(mut handler: HANDLER) -> HANDLER {
    let quantum: i32 = 0.05 as i32;
    while handler.pthread_pool.get_active_threads_number(RoundRobin)> 0 {
        handler = my_mutex_unlock(handler);
        handler = my_thread_yield(handler);
        usleep(quantum as u32);
        handler.pthread_pool.actual_thread[0].finishing_validator();
    }
    return handler;

}

//funcion que determina cual de los hilos de la lista de real time es el que debe ejecutarse
pub(crate) fn shortest_job_selector (mut handler: HANDLER) -> MyPthread {
    let mut shortest_job = handler.pthread_pool.rt_pthreads[0].clone();
    for pthread in handler.pthread_pool.rt_pthreads {
        if pthread.context.uc_stack.ss_size < shortest_job.context.uc_stack.ss_size && (pthread.state == states::ready || pthread.state == states::running) {
            shortest_job = pthread.clone();
        }
    }

    return shortest_job;
}

//EDF
//funcion que dirige el algoritm de planificacion de los hilos de real time
pub(crate) unsafe fn scheduler_real_time(mut handler: HANDLER) -> HANDLER {
    let quantum: i32 = 0.05 as i32;
    while handler.pthread_pool.get_active_threads_number(SchedulerEnum::RealTime)> 0 {
        let mut next_thread = shortest_job_selector(handler.clone());
        //se cambia el estado del thread actual a running
        handler = my_mutex_unlock(handler.clone());
        handler = my_thread_join(handler.clone() , handler.pthread_pool.get_index_by_id(next_thread.id));
        sleep(quantum as c_uint);

    }
    return handler;

}


//funcion que dirige el algoritm de planificacion de los hilos de lottery
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
    while  handler.pthread_pool.get_active_threads_number(SchedulerEnum::Lottery)> 0 {
        let indice_ticket = rng.gen_range(0..tombola.tickets.len());
        let mut winner = tombola.tickets[indice_ticket].clone();
        let mut index = handler.pthread_pool.get_index_by_id(winner.thread_id);
        let mut next_thread = handler.pthread_pool.lt_pthreads[index].clone();
        //se eliminan de la tombola todos los tiketes del ganador
        tombola.tickets.retain(|x| x.thread_id != winner.thread_id);
        if next_thread.state == states::ready {
            handler = my_mutex_unlock(handler);
            handler = my_thread_join(handler.clone(), handler.pthread_pool.get_index_by_id(next_thread.id));
            handler.pthread_pool.actual_thread[0].finishing_validator();
        }
    }
    return handler;
}




