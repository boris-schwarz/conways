use macroquad::prelude::*;
use std::time::Duration;

const SIZE: usize = 400;
const CELL_SIZE: f32 = 2.0;
const LAST: usize = SIZE - 1;
const INTERVAL: Duration = Duration::from_millis(40);

type Row = [u8; SIZE];
type Grid = [Row; SIZE];

#[macroquad::main("Conways")]
async fn main() {
    let mut active: u8 = 1;
    let mut grid_1: Grid = generate_grid();
    let mut grid_2: Grid = grid_1.clone();
    let mut r: usize = 0;
    let mut last_frame = get_time();

    request_new_screen_size(SIZE as f32 * CELL_SIZE, SIZE as f32 * CELL_SIZE);

    loop {
        let current_frame = get_time();
        let delta = current_frame - last_frame;
        clear_background(BLACK);

        if delta >= INTERVAL.as_secs_f64() {
            if active == 1 {
                calculate_next_grid(&grid_1, &mut grid_2);
                active = 2;
            } else {
                calculate_next_grid(&grid_2, &mut grid_1);
                active = 1;
            }
            r = r + 1;
            last_frame = current_frame;
        }

        if active == 1 {
            render_graphical_grid(&grid_1, &r);
        } else {
            render_graphical_grid(&grid_2, &r);
        }

        next_frame().await
    }

    fn calculate_next_grid(current_grid: &Grid, next_grid: &mut Grid) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let mut neighbours: u8 = 0;
                let cell = current_grid[y][x];

                let y_n: usize = if y > 0 { y - 1 } else { LAST };
                let x_e: usize = if x < LAST { x + 1 } else { 0 };
                let y_s: usize = if y < LAST { y + 1 } else { 0 };
                let x_w: usize = if x > 0 { x - 1 } else { LAST };

                // check north
                neighbours = neighbours + current_grid[y_n][x];

                // check north-east
                neighbours = neighbours + current_grid[y_n][x_e];

                // check east
                neighbours = neighbours + current_grid[y][x_e];

                // check south-east
                neighbours = neighbours + current_grid[y_s][x_e];

                // check south
                neighbours = neighbours + current_grid[y_s][x];

                // check south-west
                neighbours = neighbours + current_grid[y_s][x_w];

                // check west
                neighbours = neighbours + current_grid[y][x_w];

                // check north-west
                neighbours = neighbours + current_grid[y_n][x_w];

                // alive cell actions
                if cell == 1 {
                    match neighbours {
                        2 | 3 => next_grid[y][x] = 1,
                        _ => next_grid[y][x] = 0,
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

    fn render_graphical_grid(grid: &Grid, r: &usize) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let color = match grid[y][x] {
                    0 => BLACK,
                    1 => LIGHTGRAY,
                    _ => RED,
                };
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }
        draw_text(format!("{r}"), 0.0, screen_height(), 20.0, RED);
        //println!("{r}/{RUNS}");
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
            row[i] = match ::rand::random_bool(0.25) {
                true => 1,
                false => 0,
            };
        }
        row
    }
}
