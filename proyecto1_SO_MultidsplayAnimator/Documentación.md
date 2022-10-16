# Proyecto #1

## Estudiante:

- Luis Andrés Rojas Murillo
  - Carné: 2020124333

## Introducción:

En este proyecto se propuso la creación de una biblioteca de manejo de hilos de procesamiento a nivel de la capa de usuario para sistemas operativos Linux, ademas de una aplicación de animación multidisplay que corra sobre la biblioteca creada.

## Ambiente de desarrollo

- **CLion**: como IDLE para la creación del código.
- **GitHub**: como repositorio de trabajo.

## Estructuras de datos y funciones:

### Estructuras de datos:

En esta sección se puede detallar que en el código las principales estructuras usadas fueron vectores, listas y ademas os objetos de **MyPthread**, se puede decir que es el hilo como tal y la estructura **PthreadPool**, la cual es un objeto que retiene los hilos creados en los diferentes apartados según el scheduler que se les hay definido, ademas de regular todos los contenidos y datos necesarios para la ejecución del programa, también se utilizaron objetos como la tómbola o los tickets en el scheduler de Lottery pero no son estructuras vitales.

### Funciones:

#### Schedulers:

-  `scheduler_round_robin(mut pool: PthreadPool) -> PthreadPool`
-  `shortest_job_selector (mut pool: PthreadPool) -> MyPthread`
-  `scheduler_real_time(mut pool: PthreadPool) -> PthreadPool`
-  `scheduler_lottery(mut pool: PthreadPool) -> PthreadPool`

#### My_pthread_pool:

-  `create_pthread_pool() -> PthreadPool`
-  `remove_thread(mut pool: PthreadPool,mut thread_id: usize) -> PthreadPool`
-  `change_scheduler(mut pool: PthreadPool, scheduler: SchedulerEnum) -> PthreadPool`

#### My_pthread:

-  `my_thread_create(mut priority: u64, mut pool: PthreadPool, func: extern "C" fn(), mut scheduler: SchedulerEnum) -> PthreadPool`
-  `my_thread_yield(mut pool: PthreadPool) -> PthreadPool`
-  `my_thread_detach(mut thread: MyPthread, mut pool: PthreadPool) -> PthreadPool`
-  `my_thread_join(mut pool: PthreadPool, mut index: usize) -> PthreadPool`
-  `my_thread_end(mut pool: PthreadPool, index: usize) -> PthreadPool`
-  `my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread`

#### Mutex:

-  `my_mutex_init()-> bool`
-  `change_mutex_state(mut mutex: Option<bool>) -> bool`
-  `my_mutex_lock(mut pool: PthreadPool) -> PthreadPool`
-  `my_mutex_destroy(mut pool: PthreadPool) -> PthreadPool`
-  `my_mutex_unlock(mut pool: PthreadPool) -> PthreadPool`
-  `my_mutex_trylock(mut pool: PthreadPool) -> PthreadPool`
