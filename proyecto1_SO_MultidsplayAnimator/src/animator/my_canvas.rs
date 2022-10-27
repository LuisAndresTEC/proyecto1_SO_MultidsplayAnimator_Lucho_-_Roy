use libc::sleep;
use ncurses::{addstr, box_, cbreak, chtype, curs_set, echo, endwin, getch, initscr, mvprintw, mvwprintw, newwin, noecho, refresh, wborder, wclear, WINDOW, wprintw, wrefresh};
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::ll::werase;

use crate::parser::{Language, load_file, set_values};

pub(crate) unsafe fn animation(){
    let x = 0;
    let y = 0;

    //crea el canvas
    initscr();
    echo();
    curs_set(CURSOR_INVISIBLE);

    // crea una ventana con los valores del lenguaje
    let file = load_file();
    let animation_config= set_values(file);
    let  monitor = newwin(animation_config.height,animation_config.width,y,x);
    refresh();

    // se define el formato del canvas
    let corner = '+';
    let vertical = '|';
    let horizontal = '=';
    let parse_corner = chtype::from(corner);
    let parse_vertical = chtype::from(vertical);
    let parse_horizontal = chtype::from(horizontal);

    // pinta los bordes
    wborder(monitor,parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);
    refresh();
    mvwprintw(monitor, animation_config.start_position[1]-1, animation_config.start_position[0], "Posicion inicial");

    // carga el ascii en el canvas
    let ascii_size = animation_config.ascii.len();
    for i  in 0..ascii_size {
        mvwprintw(monitor, animation_config.start_position[1]+i as i32, animation_config.start_position[0], &animation_config.ascii[i]);
    }

    // refresca la ventana para ver los cambios
    wrefresh(monitor);
    sleep(2);

    let mut x_start_index = animation_config.start_position[0];
    let mut y_start_index = animation_config.start_position[1];
    let movements = 1;
    let mut time = 0;
    let mut succes = true;

    // Hasta que llegue a la posicion final
    while x_start_index != animation_config.end_position[0] || y_start_index != animation_config.end_position[1] {

        // Verifica si la figura toca algun borde
        if (x_start_index == 0 || x_start_index + animation_config.ascii[0].len() as i32 == animation_config.width) ||
            (y_start_index == 0 || y_start_index + ascii_size as i32 == animation_config.height) {
            werase(monitor);
            wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

            mvwprintw(monitor, 1, 1, "Boom! La figura chocó con el borde");
            succes = false;
            break;
        } else if time == animation_config.time_execution{
            werase(monitor);
            wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

            mvwprintw(monitor, 1, 1, "Boom! Se acabó el tiempo");
            succes = false;
            break;
        }else {

            // mover a la derecha
            if x_start_index < animation_config.end_position[0] {
                werase(monitor);
                wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

                for i in 0..ascii_size {
                    mvwprintw(monitor, y_start_index + i as i32, x_start_index + movements, &animation_config.ascii_90[i]);
                }
                x_start_index = x_start_index + movements;
                sleep(1);
                wrefresh(monitor);

                // mover a la izquierda
            } else if x_start_index > animation_config.end_position[0] {
                werase(monitor);
                wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

                for i in 0..ascii_size {
                    mvwprintw(monitor, y_start_index + i as i32, x_start_index - movements, &animation_config.ascii_270[i]);
                }
                x_start_index = x_start_index - movements;
                sleep(1);
                wrefresh(monitor);
            } else {

            }
            // mover hacia abajo
            if y_start_index < animation_config.end_position[1] {
                werase(monitor);
                wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

                for i in 0..ascii_size {
                    mvwprintw(monitor, y_start_index + movements + i as i32, x_start_index, &animation_config.ascii_180[i]);
                }
                y_start_index = y_start_index + movements;

                sleep(1);
                wrefresh(monitor);

                // mover hacia arriba
            } else if y_start_index > animation_config.end_position[1] {
                werase(monitor);
                wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);

                for i in 0..ascii_size {
                    mvwprintw(monitor, y_start_index - movements + i as i32, x_start_index, &animation_config.ascii[i]);
                }
                y_start_index = y_start_index - movements;
                sleep(1);
                wrefresh(monitor);
            } else {

            }
        }
        time+=1;
    }

    if  succes{
        werase(monitor);
        wborder(monitor, parse_vertical, parse_vertical, parse_horizontal, parse_horizontal, parse_corner, parse_corner, parse_corner, parse_corner);
        wrefresh(monitor);
        // rotar ascii
        if animation_config.rotation_angle == 90 {

            for i in 0..ascii_size {
                mvwprintw(monitor, y_start_index + i as i32, x_start_index, &animation_config.ascii_90[i]);
            }

        } else if animation_config.rotation_angle == 180 {

            for i in 0..ascii_size {
                mvwprintw(monitor, y_start_index + i as i32, x_start_index, &animation_config.ascii_180[i]);
            }

        } else if animation_config.rotation_angle == 270{
            for i in 0..ascii_size {
                mvwprintw(monitor, y_start_index + i as i32, x_start_index, &animation_config.ascii_270[i]);
            }
        } else {
            for i in 0..ascii_size {
                mvwprintw(monitor, y_start_index + i as i32, x_start_index, &animation_config.ascii[i]);
            }

        }
        mvwprintw(monitor, animation_config.end_position[1]-1, animation_config.end_position[0], "Posicion final! ");
    } else{

    }

    wrefresh(monitor);
    sleep(5);
    endwin();
}
