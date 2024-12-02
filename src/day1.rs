pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(data)?;
    let sum_b = puzzle_b(data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let pairs: Vec<(u32, u32)> = data.lines().map(extract_numbers).collect();

    let mut first_list = pairs
        .clone()
        .into_iter()
        .map(|(x, _)| x)
        .collect::<Vec<u32>>();
    let mut second_list = pairs
        .clone()
        .into_iter()
        .map(|(_, y)| y)
        .collect::<Vec<u32>>();

    first_list.sort();
    second_list.sort();

    let result = first_list
        .into_iter()
        .zip(second_list.into_iter())
        .map(difference)
        .sum();

    Ok(result)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    todo!("implement me")
}

fn extract_numbers(line: &str) -> (u32, u32) {
    const DELIMITER: &str = "   ";
    let numbers: Vec<u32> = line
        .split(DELIMITER)
        .map(|n| n.parse::<u32>().expect("expected number"))
        .collect();
    assert_eq!(numbers.len(), 2);

    (numbers[0], numbers[1])
}

fn sort_both(a: Vec<u32>, b: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    (a, b)
}

fn difference(xs: (u32, u32)) -> u32 {
    let (a, b) = xs;

    if a > b {
        return a - b;
    }

    b - a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected_output_a: u32,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![
            TestCase {
                input: String::from(
                    "3   4
4   3
2   5
1   3
3   9
3   3",
                ),
                expected_output_a: 11,
            },
            TestCase {
                input: String::from("0   0"),
                expected_output_a: 0,
            },
            TestCase {
                input: String::from("1   2"),
                expected_output_a: 1,
            },
            TestCase {
                input: String::from("2   1"),
                expected_output_a: 1,
            },
        ];
        for test_case in test_cases {
            let output = puzzle_a(&test_case.input).expect("calculation a failed");
            assert_eq!(
                test_case.expected_output_a, output,
                "input: {:?}",
                test_case.input
            )
        }
    }
}
