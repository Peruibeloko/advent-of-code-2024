type ArrayOfTuples<'a> = (Vec<&'a str>, Vec<&'a str>);
type TupleOfStrs<'a> = (&'a str, &'a str);

fn split_line(str: &str) -> (&str, &str) {
    let mut iter = str.split_ascii_whitespace();
    let l = iter.next().unwrap_or("");
    let r = iter.next().unwrap_or("");
    (l, r)
}

fn array_of_tuples_to_tuple_of_arrays<'a>(
    (mut l_arr, mut r_arr): ArrayOfTuples<'a>,
    (l, r): TupleOfStrs<'a>,
) -> ArrayOfTuples<'a> {
    l_arr.push(l);
    r_arr.push(r);
    (l_arr, r_arr)
}

pub fn solution(input: &str) -> usize {
    let (mut l_arr, mut r_arr) = input
        .lines()
        .map(split_line)
        .fold((vec![], vec![]), array_of_tuples_to_tuple_of_arrays);

    l_arr.sort();
    r_arr.sort();

    let mut r_iter = r_arr.iter();
    let mut acc = 0;

    for l in l_arr {
        let r = r_iter.next().map_or(0, |v| v.parse::<usize>().unwrap_or(0));
        acc += l.parse::<usize>().unwrap_or(0).abs_diff(r);
    }

    acc
}
