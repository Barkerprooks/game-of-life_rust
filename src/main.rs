use ncurses::*;
use std::{ thread, time };

const COL: usize = 50;
const ROW: usize = COL / 2;
const MAX: u16 = 1500;

#[derive(Copy, Clone)]
struct Point(i8, i8);

fn init_tui() {
    initscr(); cbreak(); noecho(); nonl();
    intrflush(stdscr(), false);
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn check(matrix: &mut Vec<bool>, point: Point) -> u8 {
    
    let Point(at_x, at_y) = point;
    let mut found: u8 = 0;

    for y in (at_y - 1)..=(at_y + 1) {

        if !(y >= 0 && y < ROW as i8) { continue; }

        for x in (at_x - 1)..=(at_x + 1) {
            
            if !(x >= 0 && x < COL as i8) || (x == at_x && y == at_y) { continue; } 

            if matrix[x as usize + COL * y as usize] == true { found += 1; }
        }
    }

    found
}

fn born(matrix: &mut Vec<bool>, point: Point) {
    
    let Point(x, y) = point;
    matrix[x as usize + COL * y as usize] = true;
}

fn dies(matrix: &mut Vec<bool>, point: Point) {
    
    let Point(x, y) = point; 
    matrix[x as usize + COL * y as usize] = false;
}

fn update(matrix: &mut Vec<bool>) {

    let mut is_born: Vec<Point> = Vec::new();
    let mut to_kill: Vec<Point> = Vec::new();

    for y in 0..ROW {
        for x in 0..COL {

            let point = Point(x as i8, y as i8);

            match check(matrix, point) {
                2 => (),
                3 => is_born.push(point),
                _ => to_kill.push(point)
            };
        }
    }

    for agent in is_born { born(matrix, agent); }
    for agent in to_kill { dies(matrix, agent); }
}

fn display(matrix: &Vec<bool>) {
    
    clear();

    for y in 0..ROW {
        for x in 0..COL {
            match matrix[x + COL * y] {
                true => addstr("*"),
                false => addstr(".")
            };
        }
        addstr("\n");
    }

    refresh();
    thread::sleep(time::Duration::from_millis(75));
}

fn main() {
    
    init_tui();

    let init = [[5,5], [5,6], [5,7], [6,7], [7, 6]];

    let mut matrix = vec![false; ROW * COL];
    let mut cycles = 0u16;

    for val in init.iter() {
        matrix[val[0] + COL * val[1]] = true;
    }

    while cycles <= MAX {
        
        display(&matrix);
        update(&mut matrix);
        
        cycles += 1;
    }

    endwin();
}
