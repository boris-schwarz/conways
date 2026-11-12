use macroquad::prelude::*;
use std::thread::sleep;
use std::time::Duration;

const SIZE: usize = 20;
type Row = [u8; SIZE];
type Grid = [Row; SIZE];

#[macroquad::main("MyGame")]
async fn main() {
    const SIZE: usize = 20;
    const LAST: usize = SIZE - 1;
    const RUNS: u8 = 120;
    const INTERVAL: Duration = Duration::from_millis(120);
    let mut active: u8 = 1;
    let mut grid_1: Grid = generate_grid();
    let mut grid_2: Grid = grid_1.clone();

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }

    clear();
    render_grid(&grid_1, &0);

    for r in 1..RUNS + 1 {
        sleep(INTERVAL);
        clear();
        if active % 2 == 1 {
            // println!("calculating off of grid 1, rendering grid 2...");
            calculate_next_grid(&grid_1, &mut grid_2);
            render_grid(&grid_2, &r);
            active = 2;
        } else {
            // println!("calculating off of grid 2, rendering grid 1...");
            calculate_next_grid(&grid_2, &mut grid_1);
            render_grid(&grid_1, &r);
            active = 1;
        }
    }

    fn clear() {
        print!("{esc}c", esc = 27 as char);
    }

    fn calculate_next_grid(current_grid: &Grid, next_grid: &mut Grid) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let mut neighbours: u8 = 0;
                let cell = current_grid[y][x];
                // print!("{x}:{y} = {cell}\t");

                let y_n: usize = if y > 0 { y - 1 } else { LAST };
                let x_e: usize = if x < LAST { x + 1 } else { 0 };
                let y_s: usize = if y < LAST { y + 1 } else { 0 };
                let x_w: usize = if x > 0 { x - 1 } else { LAST };

                // check north
                if current_grid[y_n][x] == 1 {
                    neighbours = neighbours + 1;
                }

                // check north-east
                if current_grid[y_n][x_e] == 1 {
                    neighbours = neighbours + 1;
                }

                // check east
                if current_grid[y][x_e] == 1 {
                    neighbours = neighbours + 1;
                }

                // check south-east
                if current_grid[y_s][x_e] == 1 {
                    neighbours = neighbours + 1;
                }

                // check south
                if current_grid[y_s][x] == 1 {
                    neighbours = neighbours + 1;
                }

                // check south-west
                if current_grid[y_s][x_w] == 1 {
                    neighbours = neighbours + 1;
                }

                // check west
                if current_grid[y][x_w] == 1 {
                    neighbours = neighbours + 1;
                }

                // check north-west
                if current_grid[y_n][x_w] == 1 {
                    neighbours = neighbours + 1;
                }

                // print!("({neighbours})\t\t");
                // alive cell actions
                if cell == 1 {
                    match neighbours {
                        2 | 3 => next_grid[y][x] = 1,
                        0 | 1 | _ => next_grid[y][x] = 0,
                    }
                // dead cell actions
                } else {
                    match neighbours {
                        3 => next_grid[y][x] = 1,
                        _ => next_grid[y][x] = 0,
                    }
                }
                /*
                 * Underpopulation: Any live cell with fewer than two live neighbors dies.
                 * Survival: Any live cell with two or three live neighbors lives on to the next generation.
                 * Overpopulation: Any live cell with more than three live neighbors dies.
                 * Reproduction: Any dead cell with exactly three live neighbors becomes a live cell.
                 */
            }
        }
    }

    fn render_grid(grid: &Grid, r: &u8) {
        // println!("Rendering grid {grid:?}");
        for row in grid {
            for cell in row {
                match cell {
                    0 => print!("  "),
                    1 => print!("██"),
                    _ => print!("__"),
                };
            }
            println!("");
        }
        println!("{r}/{RUNS}");
    }

    fn generate_grid() -> Grid {
        let mut grid: Grid = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            grid[i] = generate_row();
        }
        grid
    }

    fn generate_row() -> Row {
        let mut row: Row = [0; SIZE];
        for i in 0..SIZE {
            row[i] = match ::rand::random_bool(0.5) {
                true => 1,
                false => 0,
            };
        }
        row
    }
}
