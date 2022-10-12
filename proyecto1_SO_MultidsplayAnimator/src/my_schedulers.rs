use crate::my_pthread::{my_thread_detach, my_thread_yield, states};
<<<<<<< HEAD
use crate::my_pthread_pool::{PthreadPool, remove_thread};
use crate::schedulerEnum;
use rand::Rng;

//esta funcion no retorna nada, solo cambia el estado del hilo actual
pub(crate) unsafe fn scheduler_round_robin(mut pool: PthreadPool) -> PthreadPool {
    if pool.get_count_by_state(states::ready, schedulerEnum::round_robin) > 0 {
        pool = my_thread_yield(pool.clone());
    }
    return pool;
}

pub(crate) unsafe fn scheduler_real_time(mut pool: PthreadPool) -> PthreadPool {
    let mut next_thread = pool.rt_pthreads[pool.get_highest_priority_index().unwrap()].clone();
    if next_thread.state == states::ready {
        pool = remove_thread(pool.clone(), pool.actual_thread[0].id as usize);
        pool = my_thread_detach(next_thread, pool.clone());
    }
    return pool;
}

pub(crate) unsafe fn scheduler_lottery(mut pool: PthreadPool) -> PthreadPool {
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
    let mut tombola = tombola {
        tickets: Vec::new(),
        serial: 0,
    };
    for thread in &pool.lt_pthreads {
        for _ in 0..thread.priority {
            tombola.tickets.push(ticket {
                ticket_number: tombola.serial as u32,
                thread_id: thread.id,
            });
            tombola.serial += 1;
        }
    }
    let mut rng = rand::thread_rng();

    let mut winner = tombola.tickets[rng.gen_range(0..tombola.serial)].clone();
    let mut next_thread = pool.get_by_id(winner.thread_id).unwrap().clone();
    if next_thread.state == states::ready {
        pool = remove_thread(pool.clone(), pool.actual_thread[0].id as usize);
        pool = my_thread_detach(next_thread, pool.clone());
    }
    return pool;
}
=======
use crate::my_pthread_pool::PthreadPool;
use crate::schedulerEnum;

//esta funcion no retorna nada, solo cambia el estado del hilo actual
pub(crate) unsafe fn scheduler_round_robin(pool: PthreadPool){
    if pool.get_count_by_state(states::ready, schedulerEnum::round_robin) > 0 {
        my_thread_yield(pool);
    }
}

pub(crate) unsafe fn scheduler_real_time(pool: PthreadPool){
    let mut next_thread = pool.rt_pthreads[pool.get_highest_priority_index().unwrap()].clone();
    if next_thread.state == states::ready {
        my_thread_detach(next_thread, pool);
    }

}


/*
pub fn schedChanger(pool: PthreadPool) -> PthreadPool {
    let ActualContext = get_context(pool.actual_context);
>>>>>>> origin/main

