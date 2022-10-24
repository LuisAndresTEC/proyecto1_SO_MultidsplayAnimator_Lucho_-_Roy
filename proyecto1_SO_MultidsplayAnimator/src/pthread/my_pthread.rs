use std::mem;
use crate::my_pthread_pool::{PthreadPool, remove_thread, state_validation};
use crate::mutex::{my_mutex_lock};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, setcontext};
use crate::create_pthread_pool;
use crate::handler::{HANDLER, origin_match, secondary_match};







//este objeto es thread sobre el cual se va a trabajar
#[derive(Clone, Copy)]
pub(crate) struct MyPthread {
    pub(crate) id: u32,
    pub(crate) state: states,
    pub(crate) priority: u64,
    pub(crate) context: ucontext_t,
    pub(crate) sched: SchedulerEnum,
    pub(crate) tickets: u64,
}impl MyPthread{
    pub(crate) fn finishing_validator (mut self) -> MyPthread {
        if self.context.uc_stack.ss_size == 0 {
            self.state = states::terminated;
        }
        return self;
    }
}


//se esblecen los nombres de los diferentes tipos de schedulers
#[derive(Clone, Copy)]
pub(crate) enum SchedulerEnum {
    RealTime,
    RoundRobin,
    Lottery
}

//se establecen los estados para los threads
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum states {
    running,
    ready,
    blocked,
    terminated,
    detached
}


pub(crate) unsafe fn my_thread_create(mut priority: u64, mut handler: HANDLER, func: extern "C" fn(), mut scheduler: SchedulerEnum) -> HANDLER {
    //Se establece el context para ese nuevo thread

    unsafe {
        let mut starter: [c_char; 8192] = [mem::zeroed(); 8192];

        let mut contextCreating: ucontext_t = mem::uninitialized();
        getcontext(&mut contextCreating as *mut ucontext_t);
        contextCreating.uc_stack.ss_sp = starter.as_mut_ptr() as *mut c_void;
        contextCreating.uc_stack.ss_size = mem::size_of_val(&starter);

        contextCreating.uc_link = origin_match() as *mut ucontext_t;


        makecontext(&mut contextCreating as *mut ucontext_t, func, 0);
        //se crea el thread
        let mut thread= MyPthread {
            id: handler.serial.clone(),
            state: states::ready,
            priority: priority,
            context: contextCreating.clone(),
            sched: scheduler,
            tickets: priority,
        };
        //se agrega el thread al pool
        handler.serial += 1;
        let mut pthread_pool = handler.pthread_pool.clone();
        match thread.sched {
            SchedulerEnum::RoundRobin => {
                handler.pthread_pool.rr_pthreads.push(thread.clone());
                handler.pthread_pool.rr_contexts.push(Option::from(contextCreating.clone()));
            }
            SchedulerEnum::Lottery => {
                handler.pthread_pool.lt_pthreads.push(thread.clone());
                handler.pthread_pool.lt_contexts.push(Option::from(contextCreating.clone()));
            }
            SchedulerEnum::RealTime => {
                handler.pthread_pool.rt_pthreads.push(thread.clone());
                handler.pthread_pool.rt_contexts.push(Option::from(contextCreating.clone()));
            }
        }
    }
    return handler.clone();
}

pub(crate) unsafe fn my_thread_yield(mut handler: HANDLER) -> HANDLER {

    handler.pthread_pool.actual_thread.push(handler.pthread_pool.rr_pthreads.last().unwrap().clone());
    handler.pthread_pool.actual_context.push(handler.pthread_pool.rr_contexts.last().unwrap().clone());
    let mut thread_update= handler.pthread_pool.actual_thread[0].clone();
    let mut context_update= handler.pthread_pool.actual_context[0].clone();
    match handler.scheduler {
        SchedulerEnum::RoundRobin => {
            if state_validation(states::ready, handler.pthread_pool.rr_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rr_pthreads[0]) {
                thread_update = handler.pthread_pool.rr_pthreads[0].clone();
                context_update = handler.pthread_pool.rr_contexts[0].clone();
                thread_update.state = states::running;
                handler.pthread_pool.rr_pthreads.remove(0);
                handler.pthread_pool.rr_contexts.remove(0);

            }
        }
        SchedulerEnum::Lottery => {
            if state_validation(states::ready, handler.pthread_pool.lt_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.lt_pthreads[0]) {
                thread_update = handler.pthread_pool.lt_pthreads[0].clone();
                context_update = handler.pthread_pool.lt_contexts[0].clone();
                thread_update.state = states::running;
                handler.pthread_pool.lt_pthreads.remove(0);
                handler.pthread_pool.lt_contexts.remove(0);

            }
        }

        SchedulerEnum::RealTime => {
            if state_validation(states::ready, handler.pthread_pool.rt_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rt_pthreads[0]) {
                thread_update = handler.pthread_pool.rt_pthreads[0].clone();
                context_update = handler.pthread_pool.rt_contexts[0].clone();
                thread_update.state = states::running;
                handler.pthread_pool.rt_pthreads.remove(0);
                handler.pthread_pool.rt_contexts.remove(0);

            }
        }
    }

        //validar si  el proceso que va a salir ya termino o si hay que reintegrarlo a la cola
    if handler.mutex {
        panic!("No se puede hacer yield porque el mutex esta bloqueado");
    }else{
        swapcontext(origin_match() as *mut ucontext_t, secondary_match(0 , handler.clone()) as *const ucontext_t);
        handler = my_mutex_lock(handler.clone());
        match handler.pthread_pool.actual_thread[0].sched {
            SchedulerEnum::RealTime => {
                handler.pthread_pool.rt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.rt_contexts.push(handler.pthread_pool.actual_context[0].clone());
            }
            SchedulerEnum::RoundRobin => {
                handler.pthread_pool.rr_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.rr_contexts.push(handler.pthread_pool.actual_context[0].clone());
            }
            SchedulerEnum::Lottery => {
                handler.pthread_pool.lt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.lt_contexts.push(handler.pthread_pool.actual_context[0].clone());
            }
        }
    }
    handler.pthread_pool.actual_thread[0] = thread_update.clone();
    handler.pthread_pool.actual_context[0] = context_update.clone();

    return handler.clone();
}


pub(crate) fn my_thread_detach(mut thread: MyPthread, mut pool: PthreadPool) -> PthreadPool {
    if thread.state == states:: detached || thread.state == states::terminated || thread.state == states::blocked {
        panic!("El thread no puede ser detached");
    }else{
        let mut index = pool.get_index_by_id(thread.id).unwrap();
        match thread.sched {
            SchedulerEnum::RoundRobin => unsafe {
                setcontext(&mut thread.context as *mut ucontext_t);
                pool.rr_pthreads[index].state = states::detached;
            }
            SchedulerEnum::Lottery => unsafe {
                setcontext(&mut thread.context as *mut ucontext_t);
                pool.lt_pthreads[index].state = states::detached;

            }
            SchedulerEnum::RealTime => unsafe {
                setcontext(&mut thread.context as *mut ucontext_t);
                pool.rt_pthreads[index].state = states::detached;
            }
        }
    }
    return pool;
}




/*
//esta funcion espera a que la ejecucion de un thread termine
pub(crate) unsafe fn my_thread_join(mut pool: PthreadPool, mut index: usize) -> PthreadPool {
    let mut thread_update= pool.actual_thread[0];

    match pool.scheduler {
        SchedulerEnum::RoundRobin => {
            if state_validation(states::ready, pool.rr_pthreads[0]) ||
                state_validation(states::running, pool.rr_pthreads[0]) {
                thread_update = pool.rr_pthreads[index];
            }
        }
        SchedulerEnum::Lottery => {
            if state_validation(states::ready, pool.rr_pthreads[0]) ||
                state_validation(states::running, pool.rr_pthreads[0]) {
                thread_update = pool.lt_pthreads[index ];
            }
        }
        SchedulerEnum::RealTime => {
            if state_validation(states::ready, pool.rr_pthreads[0]) ||
                state_validation(states::running, pool.rr_pthreads[0]) {
                thread_update = pool.rt_pthreads[index ];
            }
        }
    }
    if thread_update.id == pool.actual_thread[0].id {
        panic!("No hay contextos disponibes");
    }else{
        if pool.mutex.unwrap() {
            panic!("No se puede hacer join porque el mutex esta bloqueado");
        }else {
            setcontext(&mut pool.actual_thread[index as usize].context as *mut ucontext_t);
            pool = my_mutex_lock(pool);
            match pool.actual_thread[0].sched {
                SchedulerEnum::RealTime => {
                    pool.rt_pthreads.push(pool.actual_thread[0].clone());
                }
                SchedulerEnum::RoundRobin => {
                    pool.rr_pthreads.push(pool.actual_thread[0].clone());
                }
                SchedulerEnum::Lottery => {
                    pool.lt_pthreads.push(pool.actual_thread[0].clone());
                }
            }
            pool.actual_thread[0] = thread_update.clone();
            return pool;
        }
    }

}

 */

pub(crate) fn my_thread_end(mut pool: PthreadPool, index: usize) -> PthreadPool {
    pool = remove_thread(pool, index);
    return pool
}


pub(crate) fn my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread {
    match scheduler {
        0 => thread.sched = SchedulerEnum::RealTime,
        1 => thread.sched = SchedulerEnum::RoundRobin,
        2 => thread.sched = SchedulerEnum::Lottery,
        _ => thread.sched = SchedulerEnum::RoundRobin
    }
    return thread
}











