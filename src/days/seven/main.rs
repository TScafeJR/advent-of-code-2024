use crate::days::Day;

fn parse_input_row(row: &str) -> (i64, Vec<i64>) {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let mut matches = re.captures_iter(row);

    let target = matches.next().unwrap()[1].parse::<i64>().unwrap();
    let nums = matches.map(|x| x[1].parse::<i64>().unwrap()).collect();

    return (target, nums);
}

fn is_valid_row(target: i64, nums: &[i64], curr_val: i64, p2: bool) -> bool {
    if nums.is_empty() || curr_val > target {
        return target == curr_val;
    }

    let next_val = nums[0];
    let remaining_nums = &nums[1..];

    return is_valid_row(target, remaining_nums, curr_val + next_val, p2)
        || is_valid_row(target, remaining_nums, curr_val * next_val, p2)
        || (p2
            && is_valid_row(
                target,
                remaining_nums,
                curr_val * 10i64.pow(next_val.ilog10() + 1) + next_val,
                p2,
            ));
}

fn part1(data: Vec<String>) -> u64 {
    let mut res = 0;
    for row in data {
        let (target, nums) = parse_input_row(&row);

        if is_valid_row(target, &nums[1..], nums[0], false) {
            res += target;
        }
    }

    return res as u64;
}

fn part2(data: Vec<String>) -> u64 {
    let mut res = 0;
    for row in data {
        let (target, nums) = parse_input_row(&row);

        if is_valid_row(target, &nums[1..], nums[0], true) {
            res += target;
        }
    }

    return res as u64;
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
