use crate::my_pthread::{my_thread_detach, my_thread_yield, states};
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

}*/
