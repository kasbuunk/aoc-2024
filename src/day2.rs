pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(data)?;
    let sum_b = puzzle_b(data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let matrix = parse_matrix(data);

    let num_safe_rows = matrix
        .into_iter()
        .filter(|xs| all_contain_property(xs, within_margin))
        .filter(|xs| all_contain_property(xs, decreasing) || all_contain_property(xs, increasing))
        .count();

    Ok(num_safe_rows as u32)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let matrix = parse_matrix(data);

    let num_safe_rows = matrix
        .into_iter()
        .filter(|xs| safe_row_dampened(xs.clone()))
        .count();

    Ok(num_safe_rows as u32)
}

fn parse_matrix(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|x| {
            x.split_whitespace()
                .into_iter()
                .map(|s| s.parse::<u32>().expect("str must be integer"))
                .collect()
        })
        .collect()
}

fn safe_row_dampened(row: Vec<u32>) -> bool {
    let potential_rows = list_one_left_out(row);

    potential_rows
        .into_iter()
        .filter(|xs| all_contain_property(xs, within_margin))
        .any(|xs| all_contain_property(&xs, decreasing) || all_contain_property(&xs, increasing))
}

fn list_one_left_out(row: Vec<u32>) -> Vec<Vec<u32>> {
    let mut rows = vec![row.clone()];

    for n in 0..row.len() {
        let mut row_without_element_n = row.clone();
        row_without_element_n.remove(n);
        rows.push(row_without_element_n);
    }

    rows
}

fn all_contain_property(xs: &[u32], f: fn(u32, u32) -> bool) -> bool {
    xs.iter().zip(xs.iter().skip(1)).all(|(&x, &y)| f(x, y))
}

fn decreasing(x: u32, y: u32) -> bool {
    x < y
}
fn increasing(x: u32, y: u32) -> bool {
    x > y
}

fn within_margin(x: u32, y: u32) -> bool {
    let diff = if x > y { x - y } else { y - x };

    diff >= 1 && diff <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected_output_a: u32,
        expected_output_b: u32,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![
            TestCase {
                input: String::from("1 3 2 4 5"),
                expected_output_a: 0,
                expected_output_b: 1,
            },
            TestCase {
                input: String::from("7 6 4 2 1"),
                expected_output_a: 1,
                expected_output_b: 1,
            },
            TestCase {
                input: String::from("1 2 7 8 9"),
                expected_output_a: 0,
                expected_output_b: 0,
            },
            TestCase {
                input: String::from("9 7 6 2 1"),
                expected_output_a: 0,
                expected_output_b: 0,
            },
            TestCase {
                input: String::from(
                    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
                ),
                expected_output_a: 2,
                expected_output_b: 4,
            },
        ];
        for test_case in test_cases {
            let output_a = puzzle_a(&test_case.input).expect("calculation a failed");
            assert_eq!(
                test_case.expected_output_a, output_a,
                "input: {:?}",
                test_case.input
            );

            let output_b = puzzle_b(&test_case.input).expect("calculation b failed");
            assert_eq!(
                test_case.expected_output_b, output_b,
                "input: {:?}",
                test_case.input
            );
        }
    }

    #[test]
    fn test_list_one_left_out() {
        let input = vec![1, 2, 3, 4];
        let expected_output = vec![
            vec![1, 2, 3, 4],
            vec![2, 3, 4],
            vec![1, 3, 4],
            vec![1, 2, 4],
            vec![1, 2, 3],
        ];
        let output = list_one_left_out(input);
        assert_eq!(expected_output, output);
    }
}
