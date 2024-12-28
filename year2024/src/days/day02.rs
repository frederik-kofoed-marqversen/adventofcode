pub fn run(use_test_input: bool) {
    let input = super::read_input(2, use_test_input);
    let reports: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();

    // PART 1
    println!("Result part 1: {}", reports.iter().filter(|report| is_safe(report)).count());

    // PART 2
    let mut safe_report_count = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut test = report.clone();
            test.remove(i);
            if is_safe(&test) {
                safe_report_count += 1;
                break;
            }
        }
    }
    println!("Result part 2: {safe_report_count}");
}

fn is_safe(report: &Vec<i32>) -> bool {
    let inc = report
        .windows(2)
        .map(|w| w[1] - w[0])
        .all(|val| val > 0 && val < 4);
    let dec = report
        .windows(2)
        .map(|w| w[0] - w[1])
        .all(|val| val > 0 && val < 4);

    return inc || dec
}
