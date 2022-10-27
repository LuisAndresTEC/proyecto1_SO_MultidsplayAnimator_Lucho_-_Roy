use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
#[path = "src/pthread/my_pthread"] use crate:: SchedulerEnum;

//funcion que retorna un vector con todos los elementos de un archivo txt
pub(crate) fn load_file() -> Vec<String> {
    let mut file = File::open("src/parse/message.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    return lines;
}


pub(crate) struct languaje {
    pub(crate) width: i32,
    pub(crate) height:  i32,
    pub(crate) scheduler: SchedulerEnum,
    pub(crate) ascii:  Vec<String>,
    pub(crate) time_execution: i32,
    pub(crate) start_position: Vec<i32>,
    pub(crate) end_position: Vec<i32>,
    pub(crate) rotation_angle: i32,
    pub(crate) number_of_monitors: i32,
    pub(crate) monitors:  Vec<String>,
}
pub(crate) fn set_values(file: Vec<String>) -> languaje {
    let mut start_position = Vec::new();
    let mut end_position = Vec::new();

    //se intoduce el indice 5 del file a start position como si fueran coordenadas
    let mut start = file[5].split(" ");
    let mut start_x = start.next().unwrap().parse::<i32>().unwrap();
    let mut start_y = start.next().unwrap().parse::<i32>().unwrap();
    start_position.push(start_x);
    start_position.push(start_y);

    //se intoduce el indice 6 del file a end position como si fueran coordenadas
    let mut end = file[6].split(" ");
    let mut end_x = end.next().unwrap().parse::<i32>().unwrap();
    let mut end_y = end.next().unwrap().parse::<i32>().unwrap();
    end_position.push(end_x);
    end_position.push(end_y);

    //se crea un vector con los monitores
    let mut monitors = Vec::new();
    monitors.push(file[9].to_string());
    //se crea un vertor con todos los elementos del ascii
    let mut ascii = Vec::new();
    for i in 10..17{
        ascii.push(file[i].to_string());
    }
    //se parcea de forma manual el nombre del scheduler
    let mut scheduler;
    if file[3].contains("RoundRobin") {
        scheduler = SchedulerEnum::RoundRobin;
    }else if file[3].contains("Lottery") {
        scheduler = SchedulerEnum::Lottery;
    }else {
        scheduler = SchedulerEnum::RealTime;
    }


    //se asignan los valores al struct
    let mut languaje = languaje {
        width: file[1].parse::<i32>().unwrap(),
        height: file[2].parse::<i32>().unwrap(),
        scheduler: scheduler,
        ascii: ascii,
        time_execution: file[4].parse::<i32>().unwrap(),
        start_position: start_position,
        end_position: end_position,
        rotation_angle: file[7].parse::<i32>().unwrap(),
        number_of_monitors: file[8].parse::<i32>().unwrap(),
        monitors: monitors,
    };

    return languaje;
}




//esta funcion separa el contenido del txt en un vector de strings