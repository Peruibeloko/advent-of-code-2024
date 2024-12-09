fn is_report_safe(report: &[i32]) -> bool {
    let mut tests = vec![];
    let mut results = vec![];

    for (i, _) in report.iter().enumerate() {
        let mut filtered_report = report.to_vec();
        filtered_report.remove(i);
        tests.push(filtered_report);
    }

    'outer: for report in tests {
        
        let mut last_diff: Option<i32> = None;
        for value in report.windows(2) {
            let l = *value.get(0).unwrap();
            let r = *value.get(1).unwrap();
            let diff = r - l;

            if diff.abs() < 1 || diff.abs() > 3 {
                results.push(false);
                continue 'outer;
            }

            if last_diff.is_none() {
                last_diff = Some(diff);
                continue;
            }
            
            if diff.signum() != last_diff.unwrap().signum() {
                results.push(false);
                continue 'outer;
            }

            last_diff = Some(diff);
        }
        results.push(true);
    }
    
    results.contains(&true)
}

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| is_report_safe(&report))
        .filter(|x| *x)
        .count()
}
