pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(data)?;
    let sum_b = puzzle_b(data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    todo!("implement me")
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    todo!("implement me")
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
        let test_cases = vec![TestCase {
            input: String::from(
                "3   4
4   3
2   5
1   3
3   9
3   3",
            ),
            expected_output_a: 11,
        }];
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
