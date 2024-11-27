use std::char;

fn main() {
    println!("Enter A level Grades in the format: XYZ");
    let mut input: String = std::env::args().nth(1).unwrap().parse().unwrap();

    input = input.to_uppercase();

    let grades: Vec<&str> = input.split('*').collect();

    let mut add_on: i32 = 0;
    if grades.len() > 1 {
        for i in 0..grades.len() - 1 {
            if grades[i] == "A" {
                add_on += 8;
            }
        }
    }

    let processed_grades: Vec<char> = input.chars().collect();

    let mut result: i32 = 0;
    for grade in processed_grades {
        match grade {
            'A' => result += 48,
            'B' => result += 40,
            'C' => result += 32,
            'D' => result += 24,
            'E' => result += 16,
            _ => continue,
        }
    }

    result += add_on;
    println!("{} = {}", input, result);
}
