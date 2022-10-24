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

-  `scheduler_round_robin(mut pool: PthreadPool) -> PthreadPool`: Esta función es la encargada de coordinar la ejecución del algoritmo de planificación "RoundRobin" .
-  `shortest_job_selector (mut pool: PthreadPool) -> MyPthread`
-  `scheduler_real_time(mut pool: PthreadPool) -> PthreadPool`: Esta función es la encargada de coordinar la ejecución del algoritmo de planificación  de tiempo real "EDF (earliest deadline first)"
-  `scheduler_lottery(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada de coordinar la ejecución del algoritmo de planificación "lottery", en al cual se encarga de todo el proceso de coordinación para la asignación de tiquetes y la realización de sorteo 

#### My_pthread_pool:

-  `create_pthread_pool() -> PthreadPool`:Esta función es la encargada de la creación e inicialización de la estructura de la piscina de hilos.
-  `remove_thread(mut pool: PthreadPool,mut thread_id: usize) -> PthreadPool`: Esta función es la encargada de eliminar un thread específico de la dentro de la piscina.
-  `change_scheduler(mut pool: PthreadPool, scheduler: SchedulerEnum) -> PthreadPool`: Dado que en este proyecto esta estructura de piscina tiene un atributo especifico que indica cual es el scheduler que se esta aplicando actualmente ente función se encarga de cambiar dicho atributo según los parámetros dados 

#### My_pthread:

-  `my_thread_create(mut priority: u64, mut pool: PthreadPool, func: extern "C" fn(), mut scheduler: SchedulerEnum) -> PthreadPool`:Esta función es la encargada de la creación e inicialización de la estructura hilo ademas de que tambien la agrega al threadPool
-  `my_thread_yield(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada de el reemplazo del contexto que se esta ejecutando 
-  `my_thread_detach(mut thread: MyPthread, mut pool: PthreadPool) -> PthreadPool`:**no se que poner** 
-  `my_thread_join(mut pool: PthreadPool, mut index: usize) -> PthreadPool`:Esta función es la encargada de la asignación de un hilo específico a ejecutarse
-  `my_thread_end(mut pool: PthreadPool, index: usize) -> PthreadPool`:Esta función es la encargada del cambio de estatus de un hilo y todos sus respectivos tramites de finalización
-  `my_thread_chsched(mut thread: MyPthread, scheduler: u32) -> MyPthread`:Esta función es la encargada del cambio de scheduler en un hilo específico.

#### Mutex:

-  `my_mutex_init()-> bool`:Esta función es la encargada de la creación e inicialización de la estructura del mutex
-  `change_mutex_state(mut mutex: Option<bool>) -> bool`:Esta función es la encargada del cabo de estado del mutex
-  `my_mutex_lock(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada del bloqueo del mutex
-  `my_mutex_destroy(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada de la destrucción y eliminación del mutex
-  `my_mutex_unlock(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada del desbloqueo del mutex
-  `my_mutex_trylock(mut pool: PthreadPool) -> PthreadPool`:Esta función es la encargada de bloquear el mutex en caso de que ya no lo esté.

#### Parser:

- 
