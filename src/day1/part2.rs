fn split_line(str: &str) -> (usize, usize) {
    let mut iter = str.split_ascii_whitespace();
    let l: usize = iter.next().unwrap_or("").parse().unwrap_or(0);
    let r: usize = iter.next().unwrap_or("").parse().unwrap_or(0);
    (l, r)
}

fn count_occourences(value: usize, list: &Vec<usize>) -> usize {
    list.iter().filter(|item| **item == value).count()
}

pub fn solution(input: &str) -> usize {
    let (source, count_ref) =
        input
            .lines()
            .map(split_line)
            .fold((vec![], vec![]), |(mut l_arr, mut r_arr), (l, r)| {
                l_arr.push(l);
                r_arr.push(r);
                (l_arr, r_arr)
            });

    let mut acc = 0;

    for val in source {
        acc += val * count_occourences(val, &count_ref)
    }

    acc
}
