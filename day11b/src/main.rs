use std::cmp::{max, min};

fn create_neighbor_indexes(x: usize, y: usize, x_bound: usize, y_bound: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let x_bound = x_bound as isize;
    let y_bound = y_bound as isize;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    for i in max(0, y-1)..min(y_bound, y+2) {
        for j in max(0, x-1)..min(x_bound, x+2) {
            if (j, i) != (x, y) {
                neighbors.push((j as usize, i as usize));
            }
        }
    }

    return neighbors
}

fn check_complete(octo_grid: &Vec<Vec<i32>>) -> bool {
    for i in octo_grid {
        for j in i {
            if *j != 0 {
                return false
            }
        }
    }
    return true
}

fn main() {
    let mut octo_grid: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines().map(|x| x.chars().map(|x| x as i32 - 0x30).collect()).collect();

    let mut flash_stack: Vec<(usize, usize)> = Vec::new();
    let x_bound = octo_grid[0].len();
    let y_bound = octo_grid.len();
    let mut iterations = 0;

    while !check_complete(&octo_grid) {
        iterations += 1;
        for y in 0..y_bound {
            for x in 0..x_bound {
                if octo_grid[y][x] == 9 {
                    flash_stack.append(&mut create_neighbor_indexes(x, y, x_bound, y_bound));
                    octo_grid[y][x] = 0;
                } else {
                    octo_grid[y][x] += 1;
                }
            }
        }

        while !flash_stack.is_empty() {
            let coord = flash_stack.pop().unwrap();
            if octo_grid[coord.1][coord.0] != 0 {
                if octo_grid[coord.1][coord.0] == 9 {
                    flash_stack.append(&mut create_neighbor_indexes(coord.0, coord.1, x_bound, y_bound));
                    octo_grid[coord.1][coord.0] = 0;
                } else {
                    octo_grid[coord.1][coord.0] += 1;
                }
            }
        }
    }
    println!("Synchronized: {}", iterations)
}
