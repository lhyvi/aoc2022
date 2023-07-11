use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = vec![];
    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for character in line.chars() {
            grid[i].push(character.to_digit(10).unwrap() as u8);
        }
    }
    let count_visible = count_visible_trees(&grid);
    let count_block = count_blocking_trees(&grid);
    println!("{:?} {:?}", count_visible, count_block);
}
fn count_visible_trees(grid: &Vec<Vec<u8>>) -> usize {
    let mut count = (grid.len() - 1) * 4;

    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            let height = grid[i][j];
            let mut visible = true;

            for k in (0..i).rev() {
                if grid[k][j] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for k in (i+1)..grid.len() {
                if grid[k][j] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for k in (0..j).rev() {
                if grid[i][k] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for k in (j+1)..grid[i].len() {
                if grid[i][k] >= height {
                    visible = false;
                    break;
                }
            }
            if visible {
                count += 1;
            }
        }
    }
    count
}
fn count_blocking_trees(grid: &Vec<Vec<u8>>) -> u32 {
    let mut max_product = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let height = grid[i][j];
            let mut product = 0;
            let mut count = 0;

            for k in (0..i).rev() {
                count += 1;
                if grid[k][j] >= height {
                    break;
                }
            }
            product = count;
            count = 0;

            for k in (i+1)..grid.len() {
                count += 1;
                if grid[k][j] >= height {
                    break;
                }
            }
            product *= count;
            count = 0;

            for k in (0..j).rev() {
                count += 1;
                if grid[i][k] >= height {
                    break;
                }
            }
            product *= count;
            count = 0;

            for k in (j+1)..grid[i].len() {
                count += 1;
                if grid[i][k] >= height {
                    break;
                }
            }
            product *= count;
            count = 0;

            if product > max_product {
                max_product = product;
            }
        }
    }

    max_product
}