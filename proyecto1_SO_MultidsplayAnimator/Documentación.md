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
-  `state_validation(state: states , thread: MyPthread) -> bool`: función que permite la validación si un thread está en un estado determinado.

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

- `set_values(file: Vec<String>) -> languaje`: esta función carga los datos provenientes de un vector de strings en una estructura diseñada para representar una figura y su acción 
- `load_file() -> Vec<String>`:Función que leer el txt y convierte los datos a un vector de strings

#### Handler:

- `impl HANDLER`
  - `__run_threads__(mut self)`: Método de la estructura HANDLER que dispara la ejecución de los schedulers.

- `create_handler() -> HANDLER`: función que inicializa la estructura del HANDLER.
- `origin_match() -> &'static mut ucontext_t`: función que genera un puntero a un contexto padre.
- `secondary_match(mut i:usize, handler: HANDLER) -> &'static mut ucontext_t`: función que genera un puntero desde un contexto a el contexto padre.
- `change_scheduler(mut handler: HANDLER, scheduler: SchedulerEnum) -> HANDLER`: función que cambia el scheduler del HANDLER.

#### main:

- `main()`: Función principal del código donde se crean los procesos y se define el inicio de la ejecución.

  

  ## Instrucciones para ejecutar el programa:

  

  ## Actividades realizadas por estudiante:

  |   Fecha    | Hora de inicio | Hora final | Horas invertidas | Actividad realizada                     | Autor                     |
  | :--------: | ------ | ------- | ---- | --------------------------------------- | ------------------------------ |
  | 20/09/2022 | 20:00          | 21:00      | 1:30             | Inicializar el repositorio              | Luis Andrés Rojas Murillo |
  | 26/09/2022 | 17:00          | 22:00      | 5:00             | Primeras implentaciones e investigación | Luis Andrés Rojas Murillo |
  | 28/09/2022 | 17:00 | 21:00 | 4:00 | Implementaciones e investigación | Luis Andrés Rojas Murillo |
  | 30/09/2022 | 17:00 | 20:00 | 3:00 | Primeros experimentos con los context | Luis Andrés Rojas Murillo |
  | 02/10/2022 | 9:00 | 12:00 | 3:00 | Primeras versiones de las funciones pedidas en la especificación | Luis Andrés Rojas Murillo |
  | 04/10/2022 | 18:00          | 21:00      | 3:00             | Correcciones de los errores presentes en las funciones       | Luis Andrés Rojas Murillo |
  | 07/10/2022 | 17:00 | 18:00 | 1:00 | Optimizaciones de las funciones | Luis Andrés Rojas Murillo |
  | 08/10/2022 | 11:00 | 16:00 | 5:00 | Implementación y mejora en las estructuras | Luis Andrés Rojas Murillo |
  | 09/10/2022 | 9:00 | 11:00 | 2:00 | Implementación de todas las funciones de la especificación, compilando | Luis Andrés Rojas Murillo |
  | 10/10/2022 | 12:00 | 14:00 | 2:00 | Mutex implementado en teoría y compilando | Luis Andrés Rojas Murillo |
  | 11/10/2022 | 18:00 | 21:00 | 3:00 | Validaciones medias | Luis Andrés Rojas Murillo |
  | 12/10/2022 | 9:00 | 11:00 | 2:00 | Implementación del scheduler lottery | Luis Andrés Rojas Murillo |
  | 15/10/2022 | 18:00 | 22:00 | 4:00 | Correcion de issues | Luis Andrés Rojas Murillo |
  | 16/10/2022 | 8:00 | 11:00 | 3:00 | Todos los componentes creados | Luis Andrés Rojas Murillo |
  | 18/10/2022 | 18:00 | 22:00 | 4:00 | Inicio de trabajo en parce | Luis Andrés Rojas Murillo |
  | 19/10/2022 | 8:00 | 10:00 | 2:00 | Lenguaje y parceo funcional | Luis Andrés Rojas Murillo |
  | 24/10/2022 | 16:00 | 21:00 | 5:00 | Primer intento de funcionamiento completo para RoundRobin | Luis Andrés Rojas Murillo |
  | 25/10/2022 | 16:00 | 20:00 | 4:00 | Algunas correcciones a los contextos y sus punteros | Luis Andrés Rojas Murillo |
  | 26/10/2022 | 8:00 | 10:00 | 2:00 | Reparación de problemas con punteros | Luis Andrés Rojas Murillo |
  | 26/10/2022 | 17:00 | 22:00 | 5:00 | Documentación | Luis Andrés Rojas Murillo |
  |  |  |  |  |  |  |
  |  |  |  |  |  |  |
  |            |                |            | 58:00 |                                         | Luis Andrés Rojas Murillo |
  
  #### Historial de commits:

  commit 3af3442d5f94d0065ac6a912488ee796e4c21079 (HEAD -> main, origin/main, origin/HEAD)
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Oct 26 20:05:50 2022 -0600
  
      Para el reporte de commits

  commit bac3132b8354abb3112602817ac5c34b8dc6ba95
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Oct 26 16:50:43 2022 -0600
  
      pthread final 2.0

  commit 29c78f08e7035c001fb310b6f747b2f3e177ff40
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 25 22:02:46 2022 -0600
  
      pthread final 1.0

  commit 6c8b2425f03c2980d8e06ab715812f6d0b9680d8
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Mon Oct 24 19:34:17 2022 -0600
  
      ya funcionan bien los roundRobin sin target

  commit 87eeba41f9753a6d66f3dc552cbbdd3d9a6ed2ac
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Mon Oct 24 17:54:50 2022 -0600
  
      ya funcionan bien los roundrobin

  commit b43585239b0377ff9a7d8ac782de03421a2184fa
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Oct 19 09:03:55 2022 -0600
  
      lenguaje y parceo funcional

  commit 195797badf70a5fc154797fa72e70bb8973fefdc
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 18 21:57:31 2022 -0600
  
      inicios de parce

  commit fa3071be8216330e834ba9e082a71ad51733b7c9
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 18 21:56:30 2022 -0600
  
      inicio de trabajo en parce

  commit f0ea00016a18394e19c7a27ffb93db116428a7e3
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sun Oct 16 10:58:21 2022 -0600
  
      inicios de documentación

  commit 152d2592ec8039c226e8bd50d1dd6abd50d94cd0
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sun Oct 16 10:15:29 2022 -0600
  
      Intentos de Run

  commit f81cdba83ce02ee9516d5398f77c935c56aa7a07
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sun Oct 16 09:48:29 2022 -0600
  
      Todos los componentes agregados, supuestamente

  commit e570562f71a9d2baca5eb3d261a33b9f4d92e684
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sat Oct 15 22:39:19 2022 -0600
  
      Arreglos varios

  commit 4ccc9b6b318caa83d34e614a795666c874345d10
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Oct 12 11:24:47 2022 -0600
  
      reparación del merge
  
  commit b3055fe905ea313da3f14f8fdd9757c363e51dc0
  Merge: 1c55089 8fdc417
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Oct 12 11:12:33 2022 -0600
  
      lottery scheduler implemented

  commit 1c55089e1bf3c5151f1838f7f699be4f2df3e96d
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 11 21:10:23 2022 -0600
  
      intento de implementacion de schedulers

  commit 8fdc4177aacfa6a64813aba7a9c361fc15e03a2f
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 11 21:10:23 2022 -0600
  
      intento de implementacion de schedulers

  commit a0e7f4a8bae79763fbcb0b8cdb9ef8b1f0bbfe28
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Tue Oct 11 19:08:08 2022 -0600
  
      validacion de estados para transacciones implementada

  commit 4f71ff90ea7a41c914dda6c02a2e04c2b22783b7
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Mon Oct 10 13:45:45 2022 -0600
  
      mutex implementado en teoría y compilando

  commit 2bc884aa0da04ebc8bcd26d7a1bc9d66b7aeb992
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sun Oct 9 11:12:14 2022 -0600
  
      Todas las funciones de MyPthread implementadas

  commit df20896a07793638568c380e0de664e1685432dc
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sat Oct 8 16:52:38 2022 -0600
  
      yield compilando, no se si funciona

  commit dcd90d792280311036aeb6afdfb4f9d20e0a5c2e
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sat Oct 8 16:16:19 2022 -0600
  
      optmizacion en los strucs, compilando

  commit 9f97369482189c21af18df97d85582e95e487c62
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Fri Oct 7 18:17:02 2022 -0600
  
      optmizaciones y modularizaciones

  commit 7b0a52ce72fa8ffb9efaf3c82940252d249a908c
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Sun Oct 2 12:04:34 2022 -0600
  
      funciones de yield,end,init con avances

  commit b3f52f3184fea708802ece758d2c79e16268d052
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Fri Sep 30 19:42:48 2022 -0600
  
      implementados los context en el init

  commit c4716d38b9f44ca8a519d0705a825df282711b71
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Sep 28 21:03:57 2022 -0600
  
      avances en implementaciones
  
  commit 2f1105735d9f52ae98469210efcc0fee4ef78e6b
  Merge: f4dd5b2 1f735ad
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Wed Sep 28 21:02:31 2022 -0600
  
      Merge remote-tracking branch 'origin/main'
      
      # Conflicts:
      #       proyecto1_SO_MultidsplayAnimator/src/main.rs
      #       proyecto1_SO_MultidsplayAnimator/src/my_pthread.rs

  commit f4dd5b236fb0d80ace64e701f8404f823fd37bd9
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Mon Sep 26 18:42:42 2022 -0600
  
      algunas implementaciones más

  commit 1f735ad1dc7728b56a200fd115f49d03fe4f58e2
  Author: LuisAndresTEC <lrojasmurillo7@estudiantec.cr>
  Date:   Mon Sep 26 18:42:42 2022 -0600
  
      Primeros avances

  commit 3f400c17b80045303334ad94df79f50bfd6ed94b
  Author: Luis Andrés Rojas Murillo <99442866+LuisAndresTEC@users.noreply.github.com>
  Date:   Tue Sep 20 21:10:57 2022 -0600

      Initial commit

  

  

  ## Autoevaluación:

  

  

  ## Lecciones Aprendidas del proyecto:

  

  ## Bibliografía:

  

  

  
  
  
  
  
