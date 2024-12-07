use crate::days::Day;
use crate::util::arrays;
use std::collections::HashSet;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct DirectionInfo {
    pub direction: Direction,
    pub turn_direction_char: char,
}

fn get_direction_map(d_char: char) -> Option<DirectionInfo> {
    match d_char {
        '^' => Some(DirectionInfo {
            direction: Direction::UP,
            turn_direction_char: '>',
        }),
        '>' => Some(DirectionInfo {
            direction: Direction::RIGHT,
            turn_direction_char: 'v',
        }),
        'v' => Some(DirectionInfo {
            direction: Direction::DOWN,
            turn_direction_char: '<',
        }),
        '<' => Some(DirectionInfo {
            direction: Direction::LEFT,
            turn_direction_char: '^',
        }),
        _ => None,
    }
}

fn find_guard(data: &Vec<Vec<char>>) -> (usize, usize, char) {
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if get_direction_map(data[i][j]).is_some() {
                return (i, j, data[i][j]);
            }
        }
    }

    (0, 0, data[0][0])
}

fn handle_step(data: &mut Vec<Vec<char>>, curr_loc: (usize, usize)) -> bool {
    let char = data[curr_loc.0][curr_loc.1];
    let direction_info = get_direction_map(char).unwrap();

    let (row, col) = (curr_loc.0 as i32, curr_loc.1 as i32);

    let next_move_ends = match direction_info.direction {
        Direction::UP => row - 1 < 0,
        Direction::DOWN => row + 1 == data.len() as i32,
        Direction::LEFT => col - 1 < 0,
        Direction::RIGHT => col + 1 == data[curr_loc.0].len() as i32,
    };

    if next_move_ends {
        return false;
    }

    let next_loc = match direction_info.direction {
        Direction::UP => (curr_loc.0 - 1, curr_loc.1),
        Direction::DOWN => (curr_loc.0 + 1, curr_loc.1),
        Direction::LEFT => (curr_loc.0, curr_loc.1 - 1),
        Direction::RIGHT => (curr_loc.0, curr_loc.1 + 1),
    };

    let next_char = match direction_info.direction {
        Direction::UP => data[next_loc.0][next_loc.1],
        Direction::DOWN => data[next_loc.0][next_loc.1],
        Direction::LEFT => data[next_loc.0][next_loc.1],
        Direction::RIGHT => data[next_loc.0][next_loc.1],
    };

    match next_char {
        '#' => {
            data[curr_loc.0][curr_loc.1] = direction_info.turn_direction_char;
        }
        'O' => {
            data[curr_loc.0][curr_loc.1] = direction_info.turn_direction_char;
        }
        '.' => {
            data[curr_loc.0][curr_loc.1] = '.';
            data[next_loc.0][next_loc.1] = char;
        }
        _ => {}
    }

    return true;
}

fn part1(data: Vec<String>) -> () {
    let mut converted_data: Vec<Vec<char>> =
        data.iter().map(|l| arrays::convert_str_to_vec(l)).collect();
    let mut visited_set: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let loc_resp = find_guard(&converted_data);

        let loc = (loc_resp.0, loc_resp.1);

        visited_set.insert(loc);

        let still_traversing = handle_step(&mut converted_data, loc);

        if !still_traversing {
            break;
        }
    }

    println!("day 6, part 1: {:?}", visited_set.len())
}

fn part2(data: Vec<String>) -> () {
    let converted_data: Vec<Vec<char>> =
        data.iter().map(|l| arrays::convert_str_to_vec(l)).collect();
    let mut num_loops = 0;
    let starting_loc = find_guard(&converted_data);
    let mut visited_set: HashSet<(usize, usize, char)> = HashSet::new();
    let mut loop_positions: HashSet<(usize, usize, char)> = HashSet::new();

    for i in 0..converted_data.len() {
        for j in 0..converted_data[i].len() {
            let start = Instant::now();
            let mut iter_data = converted_data.clone();
            println!("{:?}", (i, j));
            if starting_loc.0 == i && starting_loc.1 == j {
                continue;
            }

            if iter_data[i][j] == '.' {
                iter_data[i][j] = 'O';
            }

            loop {
                let loc_resp = find_guard(&iter_data);
                if visited_set.contains(&loc_resp) || loop_positions.contains(&loc_resp) {
                    num_loops += 1;
                    if loop_positions.contains(&loc_resp) {
                        loop_positions = loop_positions.union(&visited_set).cloned().collect();
                        break;
                    }
                    break;
                }

                visited_set.insert(loc_resp);

                let still_traversing = handle_step(&mut iter_data, (loc_resp.0, loc_resp.1));

                if !still_traversing {
                    break;
                }
            }
            visited_set.clear();

            let duration = start.elapsed();
            println!("Execution time: {:?}", duration);
        }
    }

    println!("day 6, part 2: {:?}", num_loops)
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
