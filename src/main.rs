use std::thread::sleep;
use std::time::Duration;

fn main() {
    const SIZE: usize = 20;
    const RUNS: u8 = 120;
    const INTERVAL: Duration = Duration::from_millis(200);
    let mut active: u8 = 1;
    let mut grid_1: [[u8; SIZE]; SIZE] = generate_grid();
    let mut grid_2 = grid_1.clone();

    print!("{esc}c", esc = 27 as char);
    render_grid(&grid_1, &0);

    for r in 1..RUNS + 1 {
        sleep(INTERVAL);
        print!("{esc}c", esc = 27 as char);
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

    fn calculate_next_grid(current_grid: &[[u8; SIZE]; SIZE], next_grid: &mut [[u8; SIZE]; SIZE]) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let mut neighbours: u8 = 0;
                let cell = current_grid[y][x];
                // print!("{x}:{y} = {cell}\t");

                // check north
                if y > 0 && current_grid[y - 1][x] == 1 || y == 0 && current_grid[SIZE - 1][x] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check north-east TODO correct this to check all wrapping neighbours
                if y > 0 && x < SIZE - 1 && current_grid[y - 1][x + 1] == 1
                    || y == 0 && x == SIZE - 1 && current_grid[SIZE - 1][0] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check east
                if x < SIZE - 1 && current_grid[y][x + 1] == 1
                    || x == SIZE - 1 && current_grid[y][0] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check south-east TODO correct this to check all wrapping neighbours
                if y < SIZE - 1 && x < SIZE - 1 && current_grid[y + 1][x + 1] == 1
                    || y == SIZE - 1 && x == SIZE - 1 && current_grid[0][0] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check south
                if y < SIZE - 1 && current_grid[y + 1][x] == 1
                    || y == SIZE - 1 && current_grid[0][x] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check south-west TODO correct this to check all wrapping neighbours
                if y < SIZE - 1 && x > 0 && current_grid[y + 1][x - 1] == 1
                    || y == SIZE - 1 && x == 0 && current_grid[0][SIZE - 1] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check west
                if x > 0 && current_grid[y][x - 1] == 1 || x == 0 && current_grid[y][SIZE - 1] == 1
                {
                    neighbours = neighbours + 1;
                }

                // check north-west TODO correct this to check all wrapping neighbours
                if y > 0 && x > 0 && current_grid[y - 1][x - 1] == 1
                    || y == 0 && x == 0 && current_grid[SIZE - 1][SIZE - 1] == 1
                {
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
            // println!("");
        }
    }

    fn render_grid(grid: &[[u8; SIZE]; SIZE], r: &u8) {
        // println!("Rendering grid {grid:?}");
        for row in grid {
            for cell in row {
                match cell {
                    0 => print!(" "),
                    1 => print!("█"),
                    _ => print!("_"),
                };
            }
            println!("");
        }
        println!("{r}/{RUNS}");
    }

    fn generate_grid() -> [[u8; SIZE]; SIZE] {
        let mut grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            grid[i] = generate_row();
        }
        grid
    }

    fn generate_row() -> [u8; SIZE] {
        let mut row: [u8; SIZE] = [0; SIZE];
        for i in 0..SIZE {
            row[i] = match rand::random_bool(0.5) {
                true => 1,
                false => 0,
            };
        }
        row
    }
}
