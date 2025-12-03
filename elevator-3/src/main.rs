use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    // let input: Vec<String> = vec![
    //     "987654321111111".to_string(),
    //     "811111111111119".to_string(),
    //     "234234234234278".to_string(),
    //     "818181911112111".to_string(),
    // ];

    let input = read_lines("input.txt");

    let mut sum = 0;

    for bank in input {
        let mut max = 0;
        let mut bank_vector: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
        for l in 0..bank_vector.len() {
            let lval = bank_vector[l];
            for r in (l + 1)..bank_vector.len() {
                let rval = bank_vector[r];
                let candidate = (format!("{}{}", lval, rval)).parse::<u32>().unwrap();
                if candidate > max {
                    max = candidate;
                }
            }
        }
        sum += max;
    }

    println!("Sum = {}", sum);
}
