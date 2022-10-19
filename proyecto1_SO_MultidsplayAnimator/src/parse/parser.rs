use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
#[path = "src/pthread/my_pthread"] use crate:: SchedulerEnum;

//funcion que retorna un vector con todos los elementos de un archivo txt
pub(crate) fn load_file() -> Vec<String> {
    let mut file = File::open("parse/message.txt").expect("file not found");
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
    pub(crate) weigth: i32,
    pub(crate) higth:  i32,
    pub(crate) scheduler: SchedulerEnum,
    pub(crate) ascii:  Vec<String>,
    pub(crate) timeExecution: i32,
    pub(crate) staryPosition: Vec<i32>,
    pub(crate) endPosition: Vec<i32>,
    pub(crate) rotationAngle: i32,
}
pub(crate) fn set_values(file: Vec<String>) -> languaje {
    let mut startPosition = Vec::new();
    let mut endPosition = Vec::new();

    //se intoduce el indice 5 del file a start position como si fueran coordenadas
    let mut start = file[5].split(" ");
    let mut start_x = start.next().unwrap().parse::<i32>().unwrap();
    let mut start_y = start.next().unwrap().parse::<i32>().unwrap();
    startPosition.push(start_x);
    startPosition.push(start_y);

    //se intoduce el indice 6 del file a end position como si fueran coordenadas
    let mut end = file[6].split(" ");
    let mut end_x = end.next().unwrap().parse::<i32>().unwrap();
    let mut end_y = end.next().unwrap().parse::<i32>().unwrap();
    endPosition.push(end_x);
    endPosition.push(end_y);

    //se crea un vertor con todos los elementos del ascii
    let mut ascii = Vec::new();
    for i in 8..15{
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
        weigth: file[1].parse::<i32>().unwrap(),
        higth: file[2].parse::<i32>().unwrap(),
        scheduler: scheduler,
        ascii: ascii,
        timeExecution: file[4].parse::<i32>().unwrap(),
        staryPosition: startPosition,
        endPosition: endPosition,
        rotationAngle: file[7].parse::<i32>().unwrap(),
    };

    return languaje;
}




//esta funcion separa el contenido del txt en un vector de strings