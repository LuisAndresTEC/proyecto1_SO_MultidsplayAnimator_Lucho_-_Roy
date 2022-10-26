use std::mem;
use crate::my_pthread_pool::{ remove_thread, state_validation};
use crate::mutex::{my_mutex_lock};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, setcontext};
use crate::create_pthread_pool;
use crate::handler::{HANDLER, origin_match, secondary_match, set_parent_context};

pub static mut CURRENT_THREAD: *mut ucontext_t = 0 as *mut ucontext_t;
pub static mut EXIT_CONTEXT: *mut ucontext_t = 0 as *mut ucontext_t;

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


    let mut thread_update= handler.pthread_pool.actual_thread[0].clone();
    let mut context_update= handler.pthread_pool.actual_context[0].clone();
    match handler.scheduler {
        SchedulerEnum::RoundRobin => {
            if state_validation(states::ready, handler.pthread_pool.rr_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rr_pthreads[0]) {
                thread_update = handler.pthread_pool.rr_pthreads[0].clone();
                context_update = handler.pthread_pool.rr_contexts[0].clone();
                thread_update.state = states::running;


            }
        }
        SchedulerEnum::Lottery => {
            if state_validation(states::ready, handler.pthread_pool.lt_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.lt_pthreads[0]) {
                thread_update = handler.pthread_pool.lt_pthreads[0].clone();
                context_update = handler.pthread_pool.lt_contexts[0].clone();
                thread_update.state = states::running;


            }
        }

        SchedulerEnum::RealTime => {
            if state_validation(states::ready, handler.pthread_pool.rt_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rt_pthreads[0]) {
                thread_update = handler.pthread_pool.rt_pthreads[0].clone();
                context_update = handler.pthread_pool.rt_contexts[0].clone();
                thread_update.state = states::running;

            }
        }
    }

        //validar si  el proceso que va a salir ya termino o si hay que reintegrarlo a la cola*/
    if handler.mutex {
        panic!("No se puede hacer yield porque el mutex esta bloqueado");
    }else{
        set_parent_context(0,handler.clone());
        CURRENT_THREAD = &mut handler.pthread_pool.rr_contexts[0].clone().unwrap();
        swapcontext(origin_match() as *mut ucontext_t, secondary_match(0 , handler.clone()) as *const ucontext_t);
        //set_parent_context(0,handler.clone());
        handler = my_mutex_lock(handler.clone());
        match handler.pthread_pool.actual_thread[0].sched {
            SchedulerEnum::RealTime => {
                /*handler.pthread_pool.rt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.rt_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                handler.pthread_pool.rt_pthreads.remove(0);
                handler.pthread_pool.rt_contexts.remove(0);
            }
            SchedulerEnum::RoundRobin => {
                /*handler.pthread_pool.rr_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.rr_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                handler.pthread_pool.rr_pthreads.remove(0);
                handler.pthread_pool.rr_contexts.remove(0);
            }
            SchedulerEnum::Lottery => {
                /*handler.pthread_pool.lt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                handler.pthread_pool.lt_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                handler.pthread_pool.lt_pthreads.remove(0);
                handler.pthread_pool.lt_contexts.remove(0);
            }
        }
    }
    /*handler.pthread_pool.actual_thread[0] = thread_update.clone();
    handler.pthread_pool.actual_context[0] = context_update.clone();*/

    return handler.clone();
}


pub(crate) fn my_thread_detach(mut thread: MyPthread, mut handler: HANDLER) -> HANDLER {
    if thread.state == states:: detached || thread.state == states::terminated || thread.state == states::blocked {
        panic!("El thread no puede ser detached");
    }else{
        let mut index = handler.pthread_pool.get_index_by_id(thread.id);
        match thread.sched {
            SchedulerEnum::RoundRobin => unsafe {
                swapcontext(origin_match() as *mut ucontext_t, secondary_match(index , handler.clone()) as *const ucontext_t);
                handler.pthread_pool.rr_pthreads[index].state = states::detached;
            }
            SchedulerEnum::Lottery => unsafe {
                swapcontext(origin_match() as *mut ucontext_t, secondary_match(index , handler.clone()) as *const ucontext_t);
                handler.pthread_pool.lt_pthreads[index].state = states::detached;

            }
            SchedulerEnum::RealTime => unsafe {
                swapcontext(origin_match() as *mut ucontext_t, secondary_match(index , handler.clone()) as *const ucontext_t);
                handler.pthread_pool.rt_pthreads[index].state = states::detached;
            }
        }
    }
    return handler;
}





//esta funcion espera a que la ejecucion de un thread termine
pub(crate) unsafe fn my_thread_join(mut handler: HANDLER, mut index: usize) -> HANDLER {
    let mut thread_update= handler.pthread_pool.actual_thread[0].clone();
    let mut context_update= handler.pthread_pool.actual_context[0].clone();

    match handler.scheduler {
        SchedulerEnum::RoundRobin => {
            if state_validation(states::ready, handler.pthread_pool.rr_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rr_pthreads[0]) {
                thread_update = handler.pthread_pool.rr_pthreads[index];
                context_update = handler.pthread_pool.rr_contexts[index];
                thread_update.state = states::terminated;
            }
        }
        SchedulerEnum::Lottery => {
            if state_validation(states::ready, handler.pthread_pool.rr_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rr_pthreads[0]) {
                thread_update = handler.pthread_pool.lt_pthreads[index];
                context_update = handler.pthread_pool.lt_contexts[index];
                thread_update.state = states::terminated;

            }
        }
        SchedulerEnum::RealTime => {
            if state_validation(states::ready, handler.pthread_pool.rr_pthreads[0]) ||
                state_validation(states::running, handler.pthread_pool.rr_pthreads[0]) {
                thread_update = handler.pthread_pool.rt_pthreads[index];
                context_update = handler.pthread_pool.rt_contexts[index];
                thread_update.state = states::terminated;

            }
        }
    }

        if handler.mutex {
            panic!("No se puede hacer join porque el mutex esta bloqueado");
        }else {
            CURRENT_THREAD = &mut handler.pthread_pool.rr_contexts[index].clone().unwrap();
            swapcontext(origin_match() as *mut ucontext_t, secondary_match(index , handler.clone()) as *const ucontext_t);
            handler = my_mutex_lock(handler.clone());
            match handler.pthread_pool.actual_thread[0].sched {
                SchedulerEnum::RealTime => {
                    /*handler.pthread_pool.rt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                    handler.pthread_pool.rt_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                    handler.pthread_pool.rt_pthreads.remove(index);
                    handler.pthread_pool.rt_contexts.remove(index);
                }
                SchedulerEnum::RoundRobin => {
                    /*handler.pthread_pool.rr_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                    handler.pthread_pool.rr_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                    handler.pthread_pool.rr_pthreads.remove(index);
                    handler.pthread_pool.rr_contexts.remove(index);
                }
                SchedulerEnum::Lottery => {
                    /*handler.pthread_pool.lt_pthreads.push(handler.pthread_pool.actual_thread[0].clone());
                    handler.pthread_pool.lt_contexts.push(handler.pthread_pool.actual_context[0].clone());*/
                    handler.pthread_pool.lt_pthreads.remove(index);
                    handler.pthread_pool.lt_contexts.remove(index);
                }
            }
            handler.pthread_pool.actual_thread[0] = thread_update.clone();
            handler.pthread_pool.actual_context[0] = context_update.clone();
            return handler;
        }


}



pub(crate) fn my_thread_end(mut handler: HANDLER, index: usize) -> HANDLER {
    handler = remove_thread(handler, index);
    return handler
}



pub(crate) fn my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread {
    match scheduler {
        1 => thread.sched = SchedulerEnum::RoundRobin,
        2 => thread.sched = SchedulerEnum::Lottery,
        3 => thread.sched = SchedulerEnum::RealTime,
        _ => thread.sched = SchedulerEnum::RoundRobin
    }
    return thread
}











