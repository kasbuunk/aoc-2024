use std::{fs::File, io::Read};

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions = vec![
        crate::day1::solution(&load_data("day1")?)?,
        crate::day2::solution(&load_data("day2")?)?,
    ];

    let solutions_report: String = solutions
        .iter()
        .enumerate()
        .map(|(index, (solution_a, solution_b))| {
            let day_number = index + 1;
            format!(
                "{}A: {}\n{}B: {}\n",
                day_number, solution_a, day_number, solution_b
            )
        })
        .collect();
    println!(
        "Solutions:
{}",
        solutions_report
    );

    Ok(())
}

fn load_data(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data_directory = "data/";
    let mut file = File::open(format!("{}{}", data_directory, file_name))?;
    let mut data = "".into();
    let _ = file.read_to_string(&mut data)?;

    Ok(data)
}

