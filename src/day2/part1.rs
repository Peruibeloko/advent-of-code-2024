#[derive(PartialEq, Debug, Clone)]
enum Direction {
    UP,
    DOWN,
}

fn is_report_safe(report: impl Iterator<Item = i32>) -> bool {
    let mut last_value = None;
    let mut last_direction = None;

    for value in report {
        if last_value.is_none() {
            last_value = Some(value);
            continue;
        }

        let diff = value.abs_diff(last_value.unwrap());
        if diff < 1 || diff > 3 {
            return false;
        }

        let direction = if value > last_value.unwrap() {
            Direction::UP
        } else {
            Direction::DOWN
        };

        match &last_direction {
            Some(last_dir) if *last_dir != direction => return false,
            None => last_direction = Some(direction),
            Some(_) => (),
        }

        last_value = Some(value);
    }

    true
}

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
        })
        .map(is_report_safe)
        .filter(|x| *x)
        .count()
}
