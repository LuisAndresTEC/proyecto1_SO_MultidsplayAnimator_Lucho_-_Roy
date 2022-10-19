use std::mem;
use crate::my_pthread_pool::{PthreadPool, remove_thread};
use crate::mutex::{my_mutex_lock};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, setcontext};
//se defien el maximo de threads que se pueden crear
const MAX_THREADS: usize = 4;

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


pub(crate) unsafe fn my_thread_create(mut priority: u64, mut pool: PthreadPool, func: extern "C" fn(), mut scheduler: SchedulerEnum) -> PthreadPool {
    //Se establece el contaxt para ese nuevo thread
    unsafe {
        let mut starter: [c_char; 8192] = [mem::zeroed(); 8192];

        let mut contextCreating: ucontext_t = mem::uninitialized();
        getcontext(&mut contextCreating as *mut ucontext_t);
        contextCreating.uc_stack.ss_sp = starter.as_mut_ptr() as *mut c_void;
        contextCreating.uc_stack.ss_size = mem::size_of_val(&starter);



        makecontext(&mut contextCreating as *mut ucontext_t, func, 0);
        //se crea el thread
        let mut thread;

        thread = MyPthread {
            id: pool.serial,
            state: states::ready,
            priority: priority,
            context: contextCreating,
            sched: scheduler,
            tickets: priority,
        };
        //se agrega el thread al pool
        pool.serial += 1;
        match thread.sched {
            SchedulerEnum::RoundRobin => {
                pool.rr_pthreads.push(thread.clone());
            }
            SchedulerEnum::Lottery => {
                pool.lt_pthreads.push(thread.clone());
            }
            SchedulerEnum::RealTime => {
                pool.rt_pthreads.push(thread.clone());
            }
        }
    }
    return pool;
}

pub(crate) unsafe fn my_thread_yield(mut pool: PthreadPool) -> PthreadPool {

    let mut thread_update= pool.actual_thread[0];
    match pool.scheduler {
        SchedulerEnum::RoundRobin => {
            if state_validation(states::ready, pool.rr_pthreads[0]) ||
                state_validation(states::running, pool.rr_pthreads[0]) {
                thread_update = pool.rr_pthreads[0].clone();
                thread_update.state = states::running;
                pool.rr_pthreads.remove(0);

            }
        }
        SchedulerEnum::Lottery => {
            if state_validation(states::ready, pool.lt_pthreads[0]) ||
                state_validation(states::running, pool.lt_pthreads[0]) {
                thread_update = pool.lt_pthreads[0].clone();
                thread_update.state = states::running;
                pool.lt_pthreads.remove(0);

            }
        }

        SchedulerEnum::RealTime => {
            if state_validation(states::ready, pool.rt_pthreads[0]) ||
                state_validation(states::running, pool.rt_pthreads[0]) {
                thread_update = pool.rt_pthreads[0].clone();
                thread_update.state = states::running;
                pool.rt_pthreads.remove(0);

            }
        }
    }
    if thread_update.id == pool.actual_thread[0].id {
        panic!("No hay contextos disponibes");
    }else{
        //validar si  el proceso que va a salir ya termino o si hay que reintegrarlo a la cola
        if pool.mutex.unwrap() {
            panic!("No se puede hacer yield porque el mutex esta bloqueado");
        }else{
            // Esto es lo que esta fallando

            //let mut context = getcontext(&mut pool.actual_thread[0].context as *mut ucontext_t);
            //swapcontext(&mut pool.actual_thread[0].context as *mut ucontext_t, &mut thread_update.context as *mut ucontext_t);
            //setcontext(&mut thread_update.context as *mut ucontext_t);
            //getcontext(&mut thread_update.context as *mut ucontext_t);
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
        }
    }

    return pool;
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


pub(crate) fn state_validation(state: states, thread: MyPthread) -> bool {
    if thread.state == state {
        return true;
    }else {
        return false;
    }
}

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

