use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut area: Vec<Vec<i32>> = vec![];
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    // parse input to area grid
    for (i, line) in input.lines().enumerate() {
        area.push(vec![]);
        for (j, character) in line.bytes().enumerate() {
            // 69 = E
            // 83 = S
            // 97 = a
            if character >= 97 {
                area[i].push(character as i32 - 97);
            } else if character == 83 {
                area[i].push(0);
                start = (i as i32, j as i32);
            } else if character == 69 {
                area[i].push(25);
                end = (i as i32, j as i32);
            }
        }
    }
    let points = get_lowest_points(&area);
    let mut point_distances = vec![];
    for point in points.iter() {
        let mut visited = vec![vec![false; area[0].len()]; area.len()];
        let mut dist = vec![vec![0i32; area[0].len()]; area.len()];
        let mut queue = VecDeque::new();
        queue.push_back(*point);
        visited[point.0 as usize][point.1 as usize] = true;

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let current_dist = dist[current.0 as usize][current.1 as usize];
            let neighbors = check_surrounding(&mut area, current.0, current.1);
            for neighbor in neighbors.iter() {
                let (y, x) = (neighbor.0 as usize, neighbor.1 as usize);
                if !visited[y][x] {
                    queue.push_back(*neighbor);
                    visited[y][x] = true;
                    dist[y][x] = current_dist + 1;
                }
            }
        }
        let end_dist = dist[end.0 as usize][end.1 as usize];
        if visited[end.0 as usize][end.1 as usize] {
            point_distances.push(end_dist);
        }
    }
    point_distances.sort();
    println!("{}", point_distances[0]);
}

const SUR: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];
fn check_surrounding(area:&mut Vec<Vec<i32>>, row: i32, column: i32) -> Vec<(i32, i32)> {
    let mut surroundings = [(0i32, 0i32); 4];
    let mut neighbors = vec![];
    for i in 0..4 {
        surroundings[i] = (SUR[i].0 + row, SUR[i].1 + column);
    }

    for neighbor in surroundings.iter() {
        if neighbor.0 < 0 || neighbor.0 > (area.len() - 1) as i32 {
            continue;
        }
        if neighbor.1 < 0 || neighbor.1 > (area[0].len() - 1) as i32 {
            continue;
        }
        let (x, y) = (neighbor.0 as usize, neighbor.1 as usize);
        if area[x][y] <= area[row as usize][column as usize] + 1 {
            neighbors.push(*neighbor);
        }
    }

    neighbors
}
fn get_lowest_points(area: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut points = vec![];
    for i in 0..area.len() {
        for j in 0..area[0].len() {
            if area[i][j] == 0 {
                points.push((i as i32, j as i32))
            }
        }
    }
    points
}
