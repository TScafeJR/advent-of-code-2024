use crate::days::Day;

#[derive(Debug)]
struct Space {
    pub open_spaces: Vec<usize>,
    pub file_spaces: Vec<usize>,
    pub spaces: Vec<usize>,
    pub first_block_idx: usize,
}

impl Space {
    pub fn new() -> Self {
        Space {
            open_spaces: Vec::new(),
            spaces: Vec::new(),
            file_spaces: Vec::new(),
            first_block_idx: 0,
        }
    }

    pub fn add_open_space(&mut self, space: usize) {
        self.open_spaces.push(space);
    }

    pub fn add_space(&mut self, space: usize) {
        self.spaces.push(space);
    }

    pub fn add_file_space(&mut self, space: usize) {
        self.file_spaces.push(space);
    }

    pub fn update_space_val(&mut self, idx: usize, val: usize) {
        self.spaces[idx] = val;
    }

    pub fn set_first_block_idx(&mut self, idx: usize) {
        self.first_block_idx = idx;
    }

    pub fn erase_file_space(&mut self) {
        self.file_spaces.pop();
    }

    pub fn erase_open_space(&mut self) {
        self.open_spaces.pop();
    }
}

fn to_int(c: char) -> u16 {
    (c as u16) - ('0' as u16)
}

fn expand_line(line: String, space: &mut Space) {
    let mut file = true;
    let mut curr_file_num = 0;
    let mut tot_idx = 0;

    for (_, c) in line.chars().enumerate() {
        let curr_left = to_int(c);

        for _ in 0..curr_left {
            if file {
                space.add_space(curr_file_num);
                space.add_file_space(tot_idx);
            } else {
                if space.first_block_idx == 0 {
                    space.set_first_block_idx(tot_idx);
                }
                space.add_space(0);
                space.add_open_space(tot_idx);
            }
            tot_idx += 1;
        }

        if file {
            curr_file_num += 1;
        }

        file = !file;
    }
}

fn get_next_num(s: Vec<usize>) -> usize {
    let mut i = 0;
    let mut c = s.clone();
    while i == 0 {
        i = c.pop().unwrap();
    }
    i
}

fn should_stop(s: Vec<usize>, leading_zeros: usize) -> bool {
    let mut c = s.clone();

    while c.last() == Some(&0) {
        c.pop();
    }

    let l = c.len();

    let filtered = c.iter().filter(|&&x| x != 0).count();

    l == filtered + leading_zeros
}

fn find_first_occurrence<T: PartialEq>(vec: Vec<T>, element: T) -> Option<usize> {
    vec.iter().position(|x| *x == element)
}

pub fn scan_for_open_spaces(
    spaces: Vec<usize>,
    leading_zeros: usize,
    curr_num: usize,
) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut count = 0;

    if let Some(idx) = find_first_occurrence(spaces.clone(), curr_num) {
        for (i, &val) in spaces.iter().enumerate() {
            if i >= idx {
                break;
            }

            if i < leading_zeros {
                continue;
            }

            if val == 0 {
                count += 1;
            } else {
                if count > 0 {
                    results.push((i - count, count));
                }
                count = 0;
            }
        }

        if count > 0 {
            results.push((idx - count, count));
        }
    }

    results
}

pub fn find_last_file_id(s: Vec<usize>) -> usize {
    let c = s.clone();
    let mut res = 0;

    for i in (0..c.len()).rev() {
        if c[i] != 0 {
            res = c[i];
            break;
        }
    }

    res
}

fn get_file_occur(s: &Vec<usize>, n: usize) -> usize {
    s.iter().filter(|&&x| x == n).count()
}

fn part1(data: Vec<String>) -> u64 {
    let mut space = Space::new();
    expand_line(data[0].clone(), &mut space);

    let mut open_spaces = space.open_spaces.clone();
    let mut spaces = space.spaces.clone();
    let mut file_spaces = space.file_spaces.clone();

    open_spaces.reverse();

    while !should_stop(spaces.clone(), space.first_block_idx) {
        let idx = open_spaces.pop().unwrap();
        let clean_up_idx = file_spaces.pop().unwrap();

        let n = get_next_num(spaces.clone());

        space.update_space_val(idx, n);
        space.update_space_val(clean_up_idx, 0);
        spaces = space.spaces.clone();
    }

    space
        .spaces
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &n)| acc + (idx as u64 * n as u64))
}

fn part2(data: Vec<String>) -> u64 {
    let mut space = Space::new();
    expand_line(data[0].clone(), &mut space);
    let mut curr_file = find_last_file_id(space.spaces.clone());

    loop {
        if curr_file == 0 || space.open_spaces.len() == 0 {
            break;
        }

        let occur = get_file_occur(&space.spaces, curr_file);
        let occur_vec =
            scan_for_open_spaces(space.spaces.clone(), space.first_block_idx, curr_file);
        let mut moved_file_info = (false, 0);

        for (idx, count) in occur_vec {
            if occur <= count {
                moved_file_info = (true, occur);
                for i_idx in idx..(idx + occur) {
                    let clean_up_idx = space.file_spaces.pop().unwrap();

                    space.erase_open_space();

                    space.update_space_val(i_idx, curr_file);
                    space.update_space_val(clean_up_idx, 0);
                }
                break;
            }
        }

        if !moved_file_info.0 {
            for _ in 0..occur {
                space.erase_file_space();
            }
        }

        curr_file -= 1;
    }

    space
        .spaces
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &n)| acc + (idx as u64 * n as u64))
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
