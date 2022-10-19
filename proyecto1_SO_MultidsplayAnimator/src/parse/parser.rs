use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
#[path = "src/pthread/my_pthread"] use crate:: SchedulerEnum;

//aqui se carga el contenido del txt
pub(crate) fn load_file() -> String {
    let mut file = File::open("src/parse/message.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    return contents;
}


pub(crate) struct languaje {
    pub(crate) weigth: i32,
    pub(crate) higth:  i32,
    pub(crate) scheduler: SchedulerEnum,
    pub(crate) ascii:  Vec<String>,
    pub(crate) timeExecution: i32,
    pub(crate) staryPosition: HashMap<i32,i32>,
    pub(crate) endPosition: HashMap<i32,i32>,
    pub(crate) rotationAngle: i32,
}
pub(crate) fn set_values(file: String)  {
    let mut lines = file.lines();
    let mut line = lines.next().unwrap();
    let mut values = line.split_whitespace();
   // values.remove(0);//quita la palabra inicio
   // values.remove(values.len() - 1);//quita la palabra fin
    //impresion de los valores
    for value in values {
        println!("{}", value);
    }

    /*let mut languaje = languaje {
        weigth:values.next().unwrap().parse().unwrap(),
        higth: values.next().unwrap().parse().unwrap(),
        scheduler: values.next().unwrap().parse().unwrap(),
        ascii: values.next().unwrap().parse().unwrap(),
        timeExecution: values.next().unwrap().parse().unwrap(),
        staryPosition: values.next().unwrap().parse().unwrap(),
        endPosition: values.next().unwrap().parse().unwrap(),
        rotationAngle: values.next().unwrap().parse().unwrap(),
    };
    return languaje;*/
}




//esta funcion separa el contenido del txt en un vector de strings